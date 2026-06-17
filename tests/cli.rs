use std::process::Command;

#[test]
fn runs_without_groups_example_csv() {
    let output = Command::new(env!("CARGO_BIN_EXE_gift_circle"))
        .arg(format!(
            "-i={}",
            env!("CARGO_MANIFEST_DIR").to_string() + "/data/example-participants-without-groups.csv"
        ))
        .output()
        .expect("failed to run gift_circle binary");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("valid utf-8 stdout");
    let mut lines = stdout.lines();
    assert_eq!(
        lines.next(),
        Some("name,email_address,group_number,assigned_person_name")
    );
    assert_eq!(lines.count(), 9);
}

#[test]
fn runs_with_groups_example_csv() {
    let output = Command::new(env!("CARGO_BIN_EXE_gift_circle"))
        .args([
            "-u",
            &format!(
                "-i={}",
                env!("CARGO_MANIFEST_DIR").to_string() + "/data/example-participants-with-groups.csv"
            ),
        ])
        .output()
        .expect("failed to run gift_circle binary");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("valid utf-8 stdout");
    let mut lines = stdout.lines();
    assert_eq!(
        lines.next(),
        Some("name,email_address,group_number,assigned_person_name")
    );
    assert_eq!(lines.count(), 9);
}
