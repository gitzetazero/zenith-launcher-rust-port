//! `instance.json`: port of `packages/instance/instance.ts` (zod) from XMCL v0.73.1.
//!
//! zod semantics reproduced here (see docs/architecture.md):
//! - `.catch(x).default(x)`: absent or invalid -> `x`.
//! - `.optional().catch(undefined)`: absent or invalid -> `None`.
//! - `z.coerce.*` use JavaScript's `String()` / `Number()`.
//! - Unknown keys are dropped; a field without `.catch` (`java`, `path`) fails the whole parse.
//!
//! shortcuts: one struct is both `InstanceSchema` (data + path + dates); XMCL's `InstanceDataSchema`
//! is the same minus those 5 fields, split it when a caller needs it. `env` keys come out sorted
//! (zod keeps insertion order). `Number()` of arrays/objects and hex strings is not reproduced
//! (the value falls back like NaN would).

use serde::{Serialize, Serializer};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

/// A zod `number`: JSON integers stay integers when written back.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct N(pub f64);

impl Serialize for N {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if self.0.fract() == 0.0 && self.0.abs() < 9_007_199_254_740_992.0 {
            s.serialize_i64(self.0 as i64)
        } else {
            s.serialize_f64(self.0)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SchemaError(pub String);

impl std::fmt::Display for SchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid instance.json: {}", self.0)
    }
}
impl std::error::Error for SchemaError {}

// ---- value parsers: Some(parsed) or None when zod would reject --------------------------------

fn p_str(v: &Value) -> Option<String> {
    v.as_str().map(str::to_owned)
}
fn p_bool(v: &Value) -> Option<bool> {
    v.as_bool()
}
fn p_num(v: &Value) -> Option<N> {
    v.as_f64().map(N)
}
fn p_positive(v: &Value) -> Option<N> {
    p_num(v).filter(|n| n.0 > 0.0)
}
fn p_nonneg(v: &Value) -> Option<N> {
    p_num(v).filter(|n| n.0 >= 0.0)
}
fn p_int_positive(v: &Value, max: f64) -> Option<f64> {
    v.as_f64()
        .filter(|n| n.fract() == 0.0 && *n > 0.0 && *n <= max)
}

/// JavaScript `String(v)`.
fn js_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "null".into(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => match n.as_f64() {
            Some(f) if f.fract() == 0.0 && f.abs() < 1e21 => format!("{}", f as i128),
            Some(f) => f.to_string(),
            None => n.to_string(),
        },
        Value::Array(a) => a
            .iter()
            .map(|e| {
                if e.is_null() {
                    String::new()
                } else {
                    js_string(e)
                }
            })
            .collect::<Vec<_>>()
            .join(","),
        Value::Object(_) => "[object Object]".into(),
    }
}

/// JavaScript `Number(v)`, `None` for NaN (zod rejects it).
fn js_number(v: &Value) -> Option<N> {
    match v {
        Value::Number(n) => n.as_f64().map(N),
        Value::Null => Some(N(0.0)),
        Value::Bool(b) => Some(N(*b as u8 as f64)),
        Value::String(s) => {
            let t = s.trim();
            if t.is_empty() {
                Some(N(0.0))
            } else {
                t.parse::<f64>().ok().filter(|f| f.is_finite()).map(N)
            }
        }
        _ => None,
    }
}

fn p_array<T>(v: &Value, f: fn(&Value) -> Option<T>) -> Option<Vec<T>> {
    v.as_array()?.iter().map(f).collect()
}
fn c_str(v: &Value) -> Option<String> {
    Some(js_string(v))
}

/// `.optional().catch(undefined)`
fn opt<T>(m: &Map<String, Value>, k: &str, f: fn(&Value) -> Option<T>) -> Option<T> {
    m.get(k).and_then(f)
}
/// `.catch(d).default(d)`
fn or<T>(m: &Map<String, Value>, k: &str, d: T, f: fn(&Value) -> Option<T>) -> T {
    m.get(k).and_then(f).unwrap_or(d)
}
/// `x.optional()` without catch: `Some(None)` absent, `Some(Some(_))` valid, `None` invalid.
fn strict<T>(m: &Map<String, Value>, k: &str, f: fn(&Value) -> Option<T>) -> Option<Option<T>> {
    match m.get(k) {
        None => Some(None),
        Some(v) => f(v).map(Some),
    }
}

// ---- runtime versions --------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVersions {
    pub minecraft: String,
    pub forge: String,
    pub neo_forged: String,
    pub fabric_loader: String,
    pub quilt_loader: String,
    pub optifine: String,
    pub laby_mod: String,
}

impl RuntimeVersions {
    fn parse(v: &Value) -> Option<Self> {
        let m = v.as_object()?;
        let s = |k| or(m, k, String::new(), p_str);
        Some(Self {
            minecraft: s("minecraft"),
            forge: s("forge"),
            neo_forged: s("neoForged"),
            fabric_loader: s("fabricLoader"),
            quilt_loader: s("quiltLoader"),
            optifine: s("optifine"),
            laby_mod: s("labyMod"),
        })
    }
}

// ---- edition -----------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Edition {
    #[default]
    Java,
    Bedrock,
}

fn p_edition(v: &Value) -> Option<Edition> {
    match v.as_str()? {
        "java" => Some(Edition::Java),
        "bedrock" => Some(Edition::Bedrock),
        _ => None,
    }
}

// ---- upstream ----------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum Upstream {
    #[serde(rename = "curseforge-modpack")]
    Curseforge {
        mod_id: N,
        file_id: N,
        #[serde(skip_serializing_if = "Option::is_none")]
        sha1: Option<String>,
    },
    #[serde(rename = "modrinth-modpack")]
    Modrinth {
        project_id: String,
        version_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        sha1: Option<String>,
    },
    #[serde(rename = "ftb-modpack")]
    Ftb { id: N, version_id: N },
    #[serde(rename = "peer")]
    Peer {
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        account_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        fingerprint: Option<String>,
    },
    #[serde(rename = "server")]
    Server {
        host: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        port: Option<N>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
}

fn p_upstream(v: &Value) -> Option<Upstream> {
    let o = v.as_object()?;
    // `z.coerce.number()` has no default: an absent key is `Number(undefined)`, NaN, rejected.
    let coerced = |k: &str| o.get(k).and_then(js_number);
    Some(match o.get("type")?.as_str()? {
        "curseforge-modpack" => Upstream::Curseforge {
            mod_id: coerced("modId")?,
            file_id: coerced("fileId")?,
            sha1: o.get("sha1").map(js_string),
        },
        "modrinth-modpack" => Upstream::Modrinth {
            project_id: p_str(o.get("projectId")?)?,
            version_id: p_str(o.get("versionId")?)?,
            sha1: strict(o, "sha1", p_str)?,
        },
        "ftb-modpack" => Upstream::Ftb {
            id: coerced("id")?,
            version_id: coerced("versionId")?,
        },
        "peer" => Upstream::Peer {
            id: p_str(o.get("id")?)?,
            account_id: strict(o, "accountId", p_str)?,
            fingerprint: strict(o, "fingerprint", p_str)?,
        },
        "server" => Upstream::Server {
            host: p_str(o.get("host")?)?,
            port: strict(o, "port", p_positive)?,
            name: strict(o, "name", p_str)?,
        },
        _ => return None,
    })
}

/// Whether two upstreams come from the same origin (`isUpstreamSameOrigin`).
pub fn is_upstream_same_origin(a: &Upstream, b: &Upstream) -> bool {
    use Upstream::*;
    match (a, b) {
        (Curseforge { mod_id: x, .. }, Curseforge { mod_id: y, .. }) => x == y,
        (Modrinth { project_id: x, .. }, Modrinth { project_id: y, .. }) => x == y,
        (Ftb { id: x, .. }, Ftb { id: y, .. }) => x == y,
        (
            Peer {
                id: i,
                account_id: a,
                ..
            },
            Peer {
                id: j,
                account_id: b,
                ..
            },
        ) => {
            // JS `a.accountId || a.id`: an empty string falls through too.
            let key = |acc: &Option<String>, id: &String| match acc {
                Some(s) if !s.is_empty() => s.clone(),
                _ => id.clone(),
            };
            key(a, i) == key(b, j)
        }
        (
            Server {
                host: h1, port: p1, ..
            },
            Server {
                host: h2, port: p2, ..
            },
        ) => {
            let port = |p: &Option<N>| p.map_or(25565.0, |n| n.0);
            h1 == h2 && port(p1) == port(p2)
        }
        _ => false,
    }
}

// ---- server config -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Policy {
    Mirror,
    Overlay,
    Initialize,
    Preserve,
}

fn p_policy(v: &Value) -> Option<Policy> {
    Some(match v.as_str()? {
        "mirror" => Policy::Mirror,
        "overlay" => Policy::Overlay,
        "initialize" => Policy::Initialize,
        "preserve" => Policy::Preserve,
        _ => return None,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WorldMode {
    Create,
    Instance,
    Preserve,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct World {
    pub mode: WorldMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Section {
    pub policy: Policy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Deployment {
    pub config: Section,
    pub mods: Section,
    pub world: Section,
    pub extra: Section,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceConfig {
    pub enabled: bool,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_command: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    pub eula: bool,
    pub motd: String,
    pub port: u32,
    pub max_players: u64,
    pub online_mode: bool,
    pub nogui: bool,
    pub world: World,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mods: Option<Vec<String>>,
    pub deployment: Deployment,
    pub service: ServiceConfig,
}

fn section(m: Option<&Map<String, Value>>, k: &str, default: Policy, with_files: bool) -> Section {
    let o = m.and_then(|m| m.get(k)).and_then(Value::as_object);
    let policy = o.map_or(default, |o| or(o, "policy", default, p_policy));
    let files = if with_files {
        o.and_then(|o| opt(o, "files", |v| p_array(v, p_str)))
    } else {
        None
    };
    Section { policy, files }
}

impl ServerConfig {
    fn parse(v: &Value) -> Option<Self> {
        let m = v.as_object()?;
        let world = m.get("world").and_then(Value::as_object);
        let world = World {
            mode: match world.and_then(|w| w.get("mode")).and_then(Value::as_str) {
                Some("instance") => WorldMode::Instance,
                Some("preserve") => WorldMode::Preserve,
                _ => WorldMode::Create,
            },
            save_name: world.and_then(|w| opt(w, "saveName", p_str)),
        };
        let d = m.get("deployment").and_then(Value::as_object);
        let svc = m.get("service").and_then(Value::as_object);
        Some(Self {
            eula: or(m, "eula", false, p_bool),
            motd: or(m, "motd", "A Minecraft Server".into(), p_str),
            port: m
                .get("port")
                .and_then(|v| p_int_positive(v, 65535.0))
                .map_or(25565, |n| n as u32),
            max_players: m
                .get("maxPlayers")
                .and_then(|v| p_int_positive(v, 9_007_199_254_740_991.0))
                .map_or(20, |n| n as u64),
            online_mode: or(m, "onlineMode", false, p_bool),
            nogui: or(m, "nogui", true, p_bool),
            world,
            mods: opt(m, "mods", |v| p_array(v, p_str)),
            deployment: Deployment {
                config: section(d, "config", Policy::Mirror, true),
                mods: section(d, "mods", Policy::Mirror, false),
                world: section(d, "world", Policy::Initialize, false),
                extra: section(d, "extra", Policy::Overlay, true),
            },
            service: ServiceConfig {
                enabled: svc.is_some_and(|s| or(s, "enabled", false, p_bool)),
                name: svc.map_or(String::new(), |s| or(s, "name", String::new(), p_str)),
                start_command: svc.and_then(|s| opt(s, "startCommand", p_str)),
            },
        })
    }
}

// ---- remote server -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AuthMethod {
    Password,
    PrivateKey,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteServer {
    pub name: String,
    pub host: String,
    pub port: N,
    pub username: String,
    pub remote_path: String,
    pub auth_method: AuthMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synced_revision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synced_at: Option<N>,
}

impl RemoteServer {
    fn parse(v: &Value) -> Option<Self> {
        let m = v.as_object()?;
        Some(Self {
            name: or(m, "name", "SSH Target".into(), p_str),
            host: p_str(m.get("host")?)?,
            port: or(m, "port", N(22.0), p_positive),
            username: p_str(m.get("username")?)?,
            remote_path: p_str(m.get("remotePath")?)?,
            auth_method: match m.get("authMethod")?.as_str()? {
                "password" => AuthMethod::Password,
                "privateKey" => AuthMethod::PrivateKey,
                _ => return None,
            },
            private_key_path: opt(m, "privateKeyPath", p_str),
            service_name: opt(m, "serviceName", p_str),
            start_command: opt(m, "startCommand", p_str),
            last_synced_revision: opt(m, "lastSyncedRevision", p_str),
            last_synced_at: opt(m, "lastSyncedAt", p_num),
        })
    }
}

// ---- the instance ------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<N>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<N>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fullscreen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<String>,
}

fn p_resolution(v: &Value) -> Option<Resolution> {
    let m = v.as_object()?;
    Some(Resolution {
        width: strict(m, "width", p_positive)?,
        height: strict(m, "height", p_positive)?,
        fullscreen: strict(m, "fullscreen", p_bool)?,
        monitor: strict(m, "monitor", p_str)?,
    })
}

/// The instance's default launch target (`server`).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ServerTarget {
    pub host: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<N>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Some(None)` is an explicit `null`.
fn p_server(v: &Value) -> Option<Option<ServerTarget>> {
    if v.is_null() {
        return Some(None);
    }
    let m = v.as_object()?;
    Some(Some(ServerTarget {
        host: p_str(m.get("host")?)?,
        port: strict(m, "port", p_positive)?,
        name: strict(m, "name", p_str)?,
    }))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignMemory {
    On,
    Auto,
    Off,
}
impl Serialize for AssignMemory {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::On => s.serialize_bool(true),
            Self::Auto => s.serialize_str("auto"),
            Self::Off => s.serialize_bool(false),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UseLatest {
    Off,
    Release,
    Alpha,
}
impl Serialize for UseLatest {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Off => s.serialize_bool(false),
            Self::Release => s.serialize_str("release"),
            Self::Alpha => s.serialize_str("alpha"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub name: String,
    pub author: String,
    pub description: String,
    pub version: String,
    pub edition: Edition,
    pub runtime: RuntimeVersions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_memory: Option<N>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_memory: Option<N>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_memory: Option<AssignMemory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mc_options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepend_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_execute_command: Option<String>,
    pub url: String,
    pub icon: String,
    pub file_api: String,
    /// `None` absent, `Some(None)` explicit null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Option<ServerTarget>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_log: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_launcher: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast_launch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_elyby_authlib: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_authlib_injector: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_game_language: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_latest: Option<UseLatest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream: Option<Upstream>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_server: Option<RemoteServer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_config: Option<ServerConfig>,
    pub last_access_date: N,
    pub last_played_date: N,
    pub playtime: N,
    pub creation_date: N,
}

impl Instance {
    /// `InstanceSchema.parse(value)`.
    pub fn parse(v: &Value) -> Result<Self, SchemaError> {
        let m = v
            .as_object()
            .ok_or_else(|| SchemaError("expected an object".into()))?;
        // `z.string().optional()` has no catch: a non-string rejects the whole instance.
        let plain = |k: &str| {
            strict(m, k, p_str).ok_or_else(|| SchemaError(format!("`{k}` must be a string")))
        };
        let zero = |k| or(m, k, N(0.0), js_number);
        Ok(Self {
            path: plain("path")?,
            name: m
                .get("name")
                .map(|v| js_string(v).trim().to_owned())
                .unwrap_or_default(),
            author: or(m, "author", String::new(), c_str),
            description: or(m, "description", String::new(), c_str),
            version: or(m, "version", String::new(), c_str),
            edition: or(m, "edition", Edition::Java, p_edition),
            runtime: or(
                m,
                "runtime",
                RuntimeVersions::default(),
                RuntimeVersions::parse,
            ),
            java: plain("java")?,
            resolution: opt(m, "resolution", p_resolution),
            min_memory: opt(m, "minMemory", p_nonneg),
            max_memory: opt(m, "maxMemory", p_positive),
            assign_memory: opt(m, "assignMemory", |v| match v {
                Value::Bool(true) => Some(AssignMemory::On),
                Value::Bool(false) => Some(AssignMemory::Off),
                Value::String(s) if s == "auto" => Some(AssignMemory::Auto),
                _ => None,
            }),
            vm_options: opt(m, "vmOptions", |v| p_array(v, c_str)),
            mc_options: opt(m, "mcOptions", |v| p_array(v, c_str)),
            env: opt(m, "env", |v| {
                Some(
                    v.as_object()?
                        .iter()
                        .map(|(k, v)| (k.clone(), js_string(v)))
                        .collect(),
                )
            }),
            prepend_command: opt(m, "prependCommand", p_str),
            pre_execute_command: opt(m, "preExecuteCommand", p_str),
            url: or(m, "url", String::new(), p_str),
            icon: or(m, "icon", String::new(), p_str),
            file_api: or(m, "fileApi", String::new(), p_str),
            server: opt(m, "server", p_server),
            show_log: opt(m, "showLog", p_bool),
            hide_launcher: opt(m, "hideLauncher", p_bool),
            fast_launch: opt(m, "fastLaunch", p_bool),
            disable_elyby_authlib: opt(m, "disableElybyAuthlib", p_bool),
            disable_authlib_injector: opt(m, "disableAuthlibInjector", p_bool),
            sync_game_language: opt(m, "syncGameLanguage", p_bool),
            use_latest: opt(m, "useLatest", |v| match v {
                Value::Bool(false) => Some(UseLatest::Off),
                Value::String(s) if s == "release" => Some(UseLatest::Release),
                Value::String(s) if s == "alpha" => Some(UseLatest::Alpha),
                _ => None,
            }),
            upstream: opt(m, "upstream", p_upstream),
            remote_server: opt(m, "remoteServer", RemoteServer::parse),
            server_config: opt(m, "serverConfig", ServerConfig::parse),
            last_access_date: zero("lastAccessDate"),
            last_played_date: zero("lastPlayedDate"),
            playtime: m
                .get("playtime")
                .and_then(js_number)
                .filter(|n| n.0 >= 0.0)
                .unwrap_or(N(0.0)),
            creation_date: zero("creationDate"),
        })
    }

    /// `isBedrockInstance`.
    pub fn is_bedrock(&self) -> bool {
        self.edition == Edition::Bedrock
    }
}

/// An instance, reduced to what `findInstanceForModpack` looks at.
pub trait ModpackMatchTarget {
    fn name(&self) -> &str;
    fn upstream(&self) -> Option<&Upstream>;
}
impl ModpackMatchTarget for Instance {
    fn name(&self) -> &str {
        &self.name
    }
    fn upstream(&self) -> Option<&Upstream> {
        self.upstream.as_ref()
    }
}

/// Finds the instance a modpack being imported corresponds to: first by upstream origin (robust to
/// renames), then by case-insensitive name (`findInstanceForModpack`).
pub fn find_instance_for_modpack<'a, T: ModpackMatchTarget>(
    instances: &'a [T],
    upstream: Option<&Upstream>,
    name: Option<&str>,
) -> Option<&'a T> {
    if let Some(up) = upstream {
        let hit = instances
            .iter()
            .find(|i| i.upstream().is_some_and(|o| is_upstream_same_origin(up, o)));
        if hit.is_some() {
            return hit;
        }
    }
    let lowered = name.filter(|n| !n.is_empty())?.to_lowercase();
    instances
        .iter()
        .find(|i| i.name().to_lowercase() == lowered)
}
