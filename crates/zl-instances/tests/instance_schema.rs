//! Translation of XMCL's `packages/instance/{instance,edition,server_config}.test.ts`, plus one test
//! per default and per `catch` rule of `instance.ts` (XMCL has none for those).

use serde_json::{json, Value};
use zl_instances::schema::*;

fn parse(v: Value) -> Instance {
    Instance::parse(&v).expect("valid")
}
fn up(v: Value) -> Upstream {
    Instance::parse(&json!({ "upstream": v }))
        .unwrap()
        .upstream
        .expect("upstream kept")
}
fn n(x: f64) -> N {
    N(x)
}

// ---- instance.test.ts: isUpstreamSameOrigin ----------------------------------------------------

#[test]
fn different_upstream_types_are_not_the_same_origin() {
    let cf = up(json!({"type":"curseforge-modpack","modId":123,"fileId":456}));
    let mr = up(
        json!({"type":"modrinth-modpack","projectId":"test-project","versionId":"test-version"}),
    );
    assert!(!is_upstream_same_origin(&cf, &mr));
}

#[test]
fn curseforge_same_mod_id_other_file() {
    let a = up(json!({"type":"curseforge-modpack","modId":123,"fileId":456}));
    let b = up(json!({"type":"curseforge-modpack","modId":123,"fileId":789}));
    let c = up(json!({"type":"curseforge-modpack","modId":456,"fileId":789}));
    assert!(is_upstream_same_origin(&a, &b));
    assert!(!is_upstream_same_origin(&a, &c));
}

#[test]
fn modrinth_same_project_other_version() {
    let a = up(json!({"type":"modrinth-modpack","projectId":"test-project","versionId":"v1.0.0"}));
    let b = up(json!({"type":"modrinth-modpack","projectId":"test-project","versionId":"v2.0.0"}));
    let c = up(json!({"type":"modrinth-modpack","projectId":"project-2","versionId":"v1.0.0"}));
    assert!(is_upstream_same_origin(&a, &b));
    assert!(!is_upstream_same_origin(&a, &c));
}

#[test]
fn ftb_same_id_other_version() {
    let a = up(json!({"type":"ftb-modpack","id":123,"versionId":1}));
    let b = up(json!({"type":"ftb-modpack","id":123,"versionId":2}));
    assert!(is_upstream_same_origin(&a, &b));
}

#[test]
fn peer_compares_account_id_then_legacy_id() {
    let a = up(
        json!({"type":"peer","id":"session-123","accountId":"account-id","fingerprint":"manifest-fingerprint"}),
    );
    let b = up(json!({"type":"peer","id":"session-456","accountId":"account-id"}));
    let c = up(json!({"type":"peer","id":"session-123","accountId":"other-account-id"}));
    assert!(is_upstream_same_origin(&a, &b));
    assert!(!is_upstream_same_origin(&a, &c));
    let legacy = up(json!({"type":"peer","id":"legacy-peer"}));
    assert!(is_upstream_same_origin(&legacy, &legacy.clone()));
}

#[test]
fn server_upstream_matches_by_host_and_port_default_25565() {
    let a = up(json!({"type":"server","host":"mc.hypixel.net"}));
    let b = up(json!({"type":"server","host":"mc.hypixel.net","port":25565}));
    let c = up(json!({"type":"server","host":"mc.hypixel.net","port":19132}));
    assert!(is_upstream_same_origin(&a, &b));
    assert!(!is_upstream_same_origin(&a, &c));
}

#[test]
fn server_upstream_never_matches_another_type() {
    let server = up(json!({"type":"server","host":"mc.hypixel.net"}));
    let peer = up(json!({"type":"peer","id":"mc.hypixel.net"}));
    assert!(!is_upstream_same_origin(&server, &peer));
}

// ---- instance.test.ts: findInstanceForModpack --------------------------------------------------

struct Target {
    path: &'static str,
    name: &'static str,
    upstream: Option<Upstream>,
}
impl ModpackMatchTarget for Target {
    fn name(&self) -> &str {
        self.name
    }
    fn upstream(&self) -> Option<&Upstream> {
        self.upstream.as_ref()
    }
}

fn instances() -> Vec<Target> {
    vec![
        Target {
            path: "/instances/optimized-mc",
            name: "Renamed by user",
            upstream: Some(up(
                json!({"type":"modrinth-modpack","projectId":"abc","versionId":"v1"}),
            )),
        },
        Target {
            path: "/instances/cf-pack",
            name: "CF Pack",
            upstream: Some(up(
                json!({"type":"curseforge-modpack","modId":123,"fileId":1}),
            )),
        },
        Target {
            path: "/instances/custom",
            name: "My Custom Pack",
            upstream: None,
        },
    ]
}

fn find(upstream: Option<Value>, name: Option<&str>) -> Option<&'static str> {
    let list = instances();
    let upstream = upstream.map(up);
    find_instance_for_modpack(&list, upstream.as_ref(), name).map(|t| t.path)
}

#[test]
fn find_matches_by_upstream_even_if_renamed() {
    let found = find(
        Some(json!({"type":"modrinth-modpack","projectId":"abc","versionId":"v2"})),
        Some("Optimized MC"),
    );
    assert_eq!(found, Some("/instances/optimized-mc"));
}

#[test]
fn find_matches_curseforge_by_mod_id_regardless_of_file_id() {
    let found = find(
        Some(json!({"type":"curseforge-modpack","modId":123,"fileId":999})),
        None,
    );
    assert_eq!(found, Some("/instances/cf-pack"));
}

#[test]
fn find_falls_back_to_case_insensitive_name() {
    assert_eq!(
        find(None, Some("my custom pack")),
        Some("/instances/custom")
    );
}

#[test]
fn find_prefers_upstream_over_name() {
    let found = find(
        Some(json!({"type":"curseforge-modpack","modId":123,"fileId":5})),
        Some("My Custom Pack"),
    );
    assert_eq!(found, Some("/instances/cf-pack"));
}

#[test]
fn find_returns_none_when_nothing_matches() {
    let found = find(
        Some(json!({"type":"modrinth-modpack","projectId":"zzz","versionId":"v1"})),
        Some("Nonexistent"),
    );
    assert_eq!(found, None);
}

#[test]
fn find_name_fallback_matches_an_instance_that_has_an_upstream() {
    assert_eq!(find(None, Some("CF Pack")), Some("/instances/cf-pack"));
}

// ---- edition.test.ts ---------------------------------------------------------------------------

#[test]
fn edition_defaults_to_java() {
    assert_eq!(parse(json!({"name":"test"})).edition, Edition::Java);
}

#[test]
fn edition_keeps_bedrock() {
    assert_eq!(
        parse(json!({"name":"test","edition":"bedrock"})).edition,
        Edition::Bedrock
    );
}

#[test]
fn edition_falls_back_to_java_when_invalid() {
    assert_eq!(
        parse(json!({"name":"test","edition":"nope"})).edition,
        Edition::Java
    );
}

#[test]
fn is_bedrock_detects_only_bedrock() {
    assert!(parse(json!({"edition":"bedrock"})).is_bedrock());
    assert!(!parse(json!({"edition":"java"})).is_bedrock());
    assert!(!parse(json!({})).is_bedrock()); // instances from before the field existed
}

// ---- server_config.test.ts ---------------------------------------------------------------------

#[test]
fn server_config_keeps_desired_state_independent_from_the_remote_result() {
    let p = parse(json!({
        "name": "server",
        "serverConfig": {
            "eula": true, "motd": "Expected state", "port": 25570, "maxPlayers": 12,
            "onlineMode": true, "nogui": true,
            "world": {"mode": "instance", "saveName": "survival"},
            "mods": ["mods/a.jar"],
            "deployment": {
                "config": {"policy": "mirror", "files": ["config/server.toml"]},
                "mods": {"policy": "mirror"},
                "world": {"policy": "initialize"},
                "extra": {"policy": "overlay", "files": ["plugins/bridge.jar"]}
            },
            "service": {"enabled": true, "name": "xmcl-survival", "startCommand": "/bin/sh server.sh"}
        },
        "remoteServer": {
            "name": "Production", "host": "example.com", "port": 22, "username": "minecraft",
            "remotePath": "/srv/minecraft", "authMethod": "privateKey"
        }
    }));
    let c = p.server_config.expect("serverConfig");
    assert_eq!(c.motd, "Expected state");
    assert_eq!(
        c.world,
        World {
            mode: WorldMode::Instance,
            save_name: Some("survival".into())
        }
    );
    assert_eq!(
        c.service,
        ServiceConfig {
            enabled: true,
            name: "xmcl-survival".into(),
            start_command: Some("/bin/sh server.sh".into())
        }
    );
    assert_eq!(
        c.deployment.config,
        Section {
            policy: Policy::Mirror,
            files: Some(vec!["config/server.toml".into()])
        }
    );
    assert_eq!(
        c.deployment.extra,
        Section {
            policy: Policy::Overlay,
            files: Some(vec!["plugins/bridge.jar".into()])
        }
    );
    assert_eq!(p.remote_server.expect("remoteServer").name, "Production");
}

#[test]
fn ssh_target_metadata_is_independent_from_deployment() {
    let p = parse(json!({
        "name": "legacy",
        "remoteServer": {"host":"example.com","username":"minecraft","remotePath":"/srv/minecraft","authMethod":"password"}
    }));
    assert_eq!(p.server_config, None);
    let r = p.remote_server.expect("remoteServer");
    assert_eq!(r.name, "SSH Target");
    assert_eq!(r.port, n(22.0));
    assert_eq!(r.auth_method, AuthMethod::Password);
    assert_eq!(r.private_key_path, None);
}

#[test]
fn service_registration_defaults_to_disabled() {
    let p = parse(json!({
        "name": "local",
        "serverConfig": {"eula": true, "motd": "Local server", "port": 25565, "maxPlayers": 20,
                         "onlineMode": false, "nogui": true, "world": {"mode": "create"}}
    }));
    let c = p.server_config.unwrap();
    assert_eq!(
        c.service,
        ServiceConfig {
            enabled: false,
            name: String::new(),
            start_command: None
        }
    );
    assert_eq!(
        c.deployment.config,
        Section {
            policy: Policy::Mirror,
            files: None
        }
    );
    assert_eq!(
        c.deployment.mods,
        Section {
            policy: Policy::Mirror,
            files: None
        }
    );
    assert_eq!(
        c.deployment.world,
        Section {
            policy: Policy::Initialize,
            files: None
        }
    );
    assert_eq!(
        c.deployment.extra,
        Section {
            policy: Policy::Overlay,
            files: None
        }
    );
}

// ---- not in XMCL's tests: defaults and catch rules ---------------------------------------------

#[test]
fn empty_object_gives_every_default() {
    let i = parse(json!({}));
    assert_eq!(
        (
            i.name.as_str(),
            i.author.as_str(),
            i.description.as_str(),
            i.version.as_str()
        ),
        ("", "", "", "")
    );
    assert_eq!(
        (i.url.as_str(), i.icon.as_str(), i.file_api.as_str()),
        ("", "", "")
    );
    assert_eq!(i.edition, Edition::Java);
    assert_eq!(i.runtime, RuntimeVersions::default());
    assert_eq!(
        (
            i.last_access_date,
            i.last_played_date,
            i.playtime,
            i.creation_date
        ),
        (n(0.0), n(0.0), n(0.0), n(0.0))
    );
    assert_eq!(i.path, None);
    assert_eq!(i.java, None);
    // every `.optional().catch(undefined)` field is absent
    let out = serde_json::to_value(&i).unwrap();
    for k in [
        "resolution",
        "minMemory",
        "maxMemory",
        "assignMemory",
        "vmOptions",
        "mcOptions",
        "env",
        "prependCommand",
        "preExecuteCommand",
        "server",
        "showLog",
        "hideLauncher",
        "fastLaunch",
        "disableElybyAuthlib",
        "disableAuthlibInjector",
        "syncGameLanguage",
        "useLatest",
        "upstream",
        "remoteServer",
        "serverConfig",
    ] {
        assert!(out.get(k).is_none(), "{k} should be absent");
    }
}

#[test]
fn default_instance_serializes_like_zod_would() {
    let out = serde_json::to_value(parse(json!({}))).unwrap();
    assert_eq!(
        out,
        json!({
            "name": "", "author": "", "description": "", "version": "", "edition": "java",
            "runtime": {"minecraft":"","forge":"","neoForged":"","fabricLoader":"","quiltLoader":"","optifine":"","labyMod":""},
            "url": "", "icon": "", "fileApi": "",
            "lastAccessDate": 0, "lastPlayedDate": 0, "playtime": 0, "creationDate": 0
        })
    );
}

#[test]
fn name_is_coerced_and_trimmed() {
    assert_eq!(parse(json!({"name": "  My pack  "})).name, "My pack");
    assert_eq!(parse(json!({"name": 42})).name, "42");
    assert_eq!(parse(json!({"name": null})).name, "null"); // String(null)
    assert_eq!(parse(json!({"name": true})).name, "true");
}

#[test]
fn plain_strings_fall_back_to_empty_when_not_strings() {
    let i = parse(json!({"url": 5, "icon": [], "fileApi": {}}));
    assert_eq!(
        (i.url.as_str(), i.icon.as_str(), i.file_api.as_str()),
        ("", "", "")
    );
}

#[test]
fn coerced_strings_use_javascript_rules() {
    let i = parse(json!({"author": 12, "description": ["a", null, 3], "version": {"x": 1}}));
    assert_eq!(i.author, "12");
    assert_eq!(i.description, "a,,3");
    assert_eq!(i.version, "[object Object]");
}

#[test]
fn runtime_fields_default_independently_and_unknown_keys_drop() {
    let i = parse(json!({"runtime": {"minecraft": "1.20.1", "forge": 3, "extra": "x"}}));
    assert_eq!(i.runtime.minecraft, "1.20.1");
    assert_eq!(i.runtime.forge, "");
    let out = serde_json::to_value(&i).unwrap();
    assert!(out["runtime"].get("extra").is_none());
}

#[test]
fn runtime_that_is_not_an_object_falls_back_to_all_empty() {
    assert_eq!(
        parse(json!({"runtime": "1.20"})).runtime,
        RuntimeVersions::default()
    );
}

#[test]
fn java_must_be_a_string_or_the_whole_parse_fails() {
    assert_eq!(
        parse(json!({"java": "C:/java/bin/java.exe"}))
            .java
            .as_deref(),
        Some("C:/java/bin/java.exe")
    );
    assert!(Instance::parse(&json!({"java": 8})).is_err());
    assert!(Instance::parse(&json!({"path": 1})).is_err());
    assert!(Instance::parse(&json!("not an object")).is_err());
}

#[test]
fn dates_and_playtime_coerce_and_fall_back_to_zero() {
    let i = parse(
        json!({"lastAccessDate": "1700", "lastPlayedDate": "abc", "creationDate": null, "playtime": -5}),
    );
    assert_eq!(i.last_access_date, n(1700.0));
    assert_eq!(i.last_played_date, n(0.0));
    assert_eq!(i.creation_date, n(0.0));
    assert_eq!(i.playtime, n(0.0)); // nonnegative, else catch(0)
    assert_eq!(parse(json!({"playtime": "90"})).playtime, n(90.0));
}

#[test]
fn memory_fields_validate_and_drop_when_invalid() {
    let i = parse(json!({"minMemory": 0, "maxMemory": 0, "assignMemory": "auto"}));
    assert_eq!(i.min_memory, Some(n(0.0))); // nonnegative accepts 0
    assert_eq!(i.max_memory, None); // positive rejects 0
    assert_eq!(i.assign_memory, Some(AssignMemory::Auto));
    assert_eq!(
        parse(json!({"assignMemory": true})).assign_memory,
        Some(AssignMemory::On)
    );
    assert_eq!(
        parse(json!({"assignMemory": false})).assign_memory,
        Some(AssignMemory::Off)
    );
    assert_eq!(parse(json!({"assignMemory": "yes"})).assign_memory, None);
    assert_eq!(parse(json!({"minMemory": -1})).min_memory, None);
    assert_eq!(parse(json!({"maxMemory": "4096"})).max_memory, None); // not coerced
}

#[test]
fn option_lists_coerce_elements_and_drop_when_not_arrays() {
    let i = parse(json!({"vmOptions": ["-Xmx2G", 5], "mcOptions": "--demo"}));
    assert_eq!(i.vm_options, Some(vec!["-Xmx2G".into(), "5".into()]));
    assert_eq!(i.mc_options, None);
}

#[test]
fn env_coerces_values_and_drops_when_not_an_object() {
    let i = parse(json!({"env": {"A": 1, "B": "x"}}));
    let env = i.env.unwrap();
    assert_eq!((env["A"].as_str(), env["B"].as_str()), ("1", "x"));
    assert_eq!(parse(json!({"env": ["A"]})).env, None);
}

#[test]
fn resolution_is_dropped_entirely_when_a_field_is_invalid() {
    let ok = parse(json!({"resolution": {"width": 1280, "height": 720, "fullscreen": false}}));
    assert_eq!(
        ok.resolution,
        Some(Resolution {
            width: Some(n(1280.0)),
            height: Some(n(720.0)),
            fullscreen: Some(false),
            monitor: None
        })
    );
    assert_eq!(parse(json!({"resolution": {"width": 0}})).resolution, None);
    assert_eq!(
        parse(json!({"resolution": {"width": 800, "monitor": 1}})).resolution,
        None
    );
}

#[test]
fn server_keeps_an_explicit_null_and_drops_invalid() {
    assert_eq!(parse(json!({})).server, None);
    assert_eq!(parse(json!({"server": null})).server, Some(None));
    let s = parse(json!({"server": {"host": "mc.example.com", "port": 25566, "name": "Home"}}));
    assert_eq!(
        s.server,
        Some(Some(ServerTarget {
            host: "mc.example.com".into(),
            port: Some(n(25566.0)),
            name: Some("Home".into())
        }))
    );
    assert_eq!(parse(json!({"server": {"port": 25565}})).server, None); // host is required
    assert_eq!(
        parse(json!({"server": {"host": "h", "port": 0}})).server,
        None
    );
}

#[test]
fn flags_are_booleans_only() {
    let i = parse(
        json!({"showLog": true, "hideLauncher": "yes", "fastLaunch": 1, "syncGameLanguage": false}),
    );
    assert_eq!(
        (
            i.show_log,
            i.hide_launcher,
            i.fast_launch,
            i.sync_game_language
        ),
        (Some(true), None, None, Some(false))
    );
}

#[test]
fn use_latest_accepts_false_release_alpha() {
    assert_eq!(
        parse(json!({"useLatest": false})).use_latest,
        Some(UseLatest::Off)
    );
    assert_eq!(
        parse(json!({"useLatest": "release"})).use_latest,
        Some(UseLatest::Release)
    );
    assert_eq!(
        parse(json!({"useLatest": "alpha"})).use_latest,
        Some(UseLatest::Alpha)
    );
    assert_eq!(parse(json!({"useLatest": true})).use_latest, None);
    assert_eq!(parse(json!({"useLatest": "beta"})).use_latest, None);
}

#[test]
fn upstream_coerces_ids_and_is_dropped_when_invalid() {
    let cf = up(json!({"type": "curseforge-modpack", "modId": "123", "fileId": "456", "sha1": 7}));
    assert_eq!(
        cf,
        Upstream::Curseforge {
            mod_id: n(123.0),
            file_id: n(456.0),
            sha1: Some("7".into())
        }
    );
    let bad = |v: Value| Instance::parse(&json!({ "upstream": v })).unwrap().upstream;
    assert_eq!(
        bad(json!({"type": "curseforge-modpack", "modId": "x", "fileId": 1})),
        None
    );
    assert_eq!(
        bad(json!({"type": "curseforge-modpack", "fileId": 1})),
        None
    ); // Number(undefined) is NaN
    assert_eq!(
        bad(json!({"type": "modrinth-modpack", "projectId": "p"})),
        None
    );
    assert_eq!(
        bad(json!({"type": "modrinth-modpack", "projectId": "p", "versionId": "v", "sha1": 5})),
        None
    );
    assert_eq!(bad(json!({"type": "unknown"})), None);
    assert_eq!(
        bad(json!({"type": "server", "host": "h", "port": -1})),
        None
    );
}

#[test]
fn server_config_defaults_and_catches() {
    let c = parse(json!({"serverConfig": {}})).server_config.unwrap();
    assert!(!c.eula && !c.online_mode && c.nogui);
    assert_eq!(
        (c.motd.as_str(), c.port, c.max_players),
        ("A Minecraft Server", 25565, 20)
    );
    assert_eq!(
        c.world,
        World {
            mode: WorldMode::Create,
            save_name: None
        }
    );
    assert_eq!(c.mods, None);

    let c = parse(json!({"serverConfig": {"port": 70000, "maxPlayers": 0, "motd": 3, "world": {"mode": "bad"},
                                          "mods": [1], "deployment": {"world": {"policy": "nope"}, "extra": 5}}}))
        .server_config
        .unwrap();
    assert_eq!(
        (c.port, c.max_players, c.motd.as_str()),
        (25565, 20, "A Minecraft Server")
    );
    assert_eq!(c.world.mode, WorldMode::Create);
    assert_eq!(c.mods, None);
    assert_eq!(c.deployment.world.policy, Policy::Initialize);
    assert_eq!(c.deployment.extra.policy, Policy::Overlay);
    assert_eq!(parse(json!({"serverConfig": 5})).server_config, None);
}

#[test]
fn remote_server_requires_its_core_fields_but_defaults_the_rest() {
    let missing =
        parse(json!({"remoteServer": {"host": "h", "username": "u", "authMethod": "password"}}));
    assert_eq!(missing.remote_server, None); // remotePath absent
    let bad_auth = parse(
        json!({"remoteServer": {"host":"h","username":"u","remotePath":"/p","authMethod":"token"}}),
    );
    assert_eq!(bad_auth.remote_server, None);
    let r = parse(json!({"remoteServer": {"host":"h","username":"u","remotePath":"/p","authMethod":"privateKey","port":-1,"name":5}}))
        .remote_server
        .unwrap();
    assert_eq!((r.port, r.name.as_str()), (n(22.0), "SSH Target"));
}

#[test]
fn integers_stay_integers_when_written_back() {
    let i = parse(json!({"maxMemory": 4096, "resolution": {"width": 1280}}));
    let out = serde_json::to_string(&i).unwrap();
    assert!(
        out.contains(r#""maxMemory":4096"#) && out.contains(r#""width":1280"#),
        "{out}"
    );
}
