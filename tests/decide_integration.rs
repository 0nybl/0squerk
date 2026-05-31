use std::fs;
use std::process::Command;

#[test]
fn decide_writes_proceed_true() {
    let work = tempfile::tempdir().unwrap();
    let event = work.path().join("event.json");
    fs::write(
        &event,
        r#"{ "issue": { "number": 3, "pull_request": {"url":"x"} },
             "comment": { "body": "/merge", "author_association": "OWNER" } }"#,
    )
    .unwrap();
    // config file intentionally absent -> default command
    let config = work.path().join(".0merge.toml");
    let out = work.path().join("decision.json");

    let status = Command::new(env!("CARGO_BIN_EXE_0squerk"))
        .args([
            "decide",
            "--event",
            event.to_str().unwrap(),
            "--config",
            config.to_str().unwrap(),
            "--out",
            out.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert!(status.success());

    let v: serde_json::Value = serde_json::from_str(&fs::read_to_string(&out).unwrap()).unwrap();
    assert_eq!(v["proceed"], true);
    assert_eq!(v["command"], "cargo test --all");
}
