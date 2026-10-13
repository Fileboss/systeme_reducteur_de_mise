use assert_cmd::Command;
use predicates::prelude::*;

fn srm() -> Command {
    Command::cargo_bin("srm").unwrap()
}

#[test]
fn play_lists_grids_and_honest_odds() {
    srm()
        .args([
            "play", "3", "7", "12", "18", "21", "25", "30", "33", "41", "44", "49", "50",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Numéros (12) : 03 07 12"))
        .stdout(predicate::str::contains("Perte moyenne attendue"))
        .stdout(predicate::str::contains("   1. 03,"));
}

#[test]
fn play_accepts_commas_and_json() {
    let out = srm()
        .args(["play", "1,2,3,4,5,6,7,8,9,10,11", "-g", "2", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(v["t"], 2);
    assert_eq!(v["numbers"].as_array().unwrap().len(), 11);
}

#[test]
fn play_rejects_bad_numbers() {
    srm()
        .args(["play", "1", "2", "3", "4", "5", "6", "51"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("51"));
    srm()
        .args(["play", "1", "2", "3", "4", "5", "6", "6"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("double"));
    srm()
        .args(["play", "1", "2", "3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("non supportés"));
}

#[test]
fn play_save_writes_a_fresh_file() {
    let dir = std::env::temp_dir().join(format!("srm-test-{}", std::process::id()));
    srm()
        .args([
            "play", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "--save",
        ])
        .arg(&dir)
        .assert()
        .success();
    let files: Vec<_> = std::fs::read_dir(&dir).unwrap().flatten().collect();
    assert_eq!(files.len(), 1);
    let text = std::fs::read_to_string(files[0].path()).unwrap();
    assert!(text.starts_with("# Numéros (10)"));
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn suggest_and_tables() {
    srm()
        .args(["suggest", "--mise", "100"])
        .assert()
        .success()
        .stdout(predicate::str::contains("numéros, garantie"));
    srm()
        .args(["suggest", "--mise", "0.01"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Aucun système"));
    srm()
        .args(["tables"])
        .assert()
        .success()
        .stdout(predicate::str::contains("grilles"))
        .stdout(predicate::str::is_match(r"(?m)^\s+30\s+4\s+\d+").unwrap());
}

#[test]
fn verify_detects_valid_and_invalid_files() {
    let legacy = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../SystReducV2/Ressources/data_20_3.txt"
    );
    srm()
        .args(["verify", legacy, "-n", "20", "-t", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("OK : 60 grilles"));
    srm()
        .args(["verify", legacy, "-n", "20", "-t", "4"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("n'est pas couvert"));
}

#[test]
fn generate_small_table_into_temp_dir() {
    let dir = std::env::temp_dir().join(format!("srm-gen-{}", std::process::id()));
    srm()
        .args([
            "generate", "-n", "10", "-t", "3", "--budget", "0.5", "--runs", "2", "--out",
        ])
        .arg(&dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("10/3"));
    let text = std::fs::read_to_string(dir.join("10_3.txt")).unwrap();
    assert!(text.starts_with("# srm"));
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn stats_compares_with_random() {
    srm()
        .args([
            "stats", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("système"))
        .stdout(predicate::str::contains("hasard"));
}
