use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("--help");
    cmd.assert().success();
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("--version");
    cmd.assert().success();
}

#[test]
fn test_get_command_help() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("get").arg("--help");
    cmd.assert().success();
}

#[test]
fn test_post_command_help() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("post").arg("--help");
    cmd.assert().success();
}

#[test]
fn test_put_command_help() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("put").arg("--help");
    cmd.assert().success();
}

#[test]
fn test_delete_command_help() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("delete").arg("--help");
    cmd.assert().success();
}

#[test]
fn test_patch_command_help() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("patch").arg("--help");
    cmd.assert().success();
}

#[test]
fn test_head_command_help() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("head").arg("--help");
    cmd.assert().success();
}

#[test]
fn test_options_command_help() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("options").arg("--help");
    cmd.assert().success();
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("invalid").arg("https://example.com");
    cmd.assert().failure();
}

#[test]
fn test_get_missing_url() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("get");
    cmd.assert().failure();
}

#[test]
fn test_post_missing_url() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("post");
    cmd.assert().failure();
}

#[test]
fn test_verbose_flag() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("--verbose").arg("get").arg("--help");
    cmd.assert().success();
}

#[test]
fn test_quiet_flag() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("--quiet").arg("get").arg("--help");
    cmd.assert().success();
}

#[test]
fn test_config_flag() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("--config")
        .arg("/tmp/config.toml")
        .arg("get")
        .arg("--help");
    cmd.assert().success();
}

#[test]
fn test_get_with_header() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("get")
        .arg("https://example.com")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("--quiet");
    cmd.assert().success();
}

#[test]
fn test_get_with_auth() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("get")
        .arg("https://example.com")
        .arg("-u")
        .arg("user:password")
        .arg("--quiet");
    cmd.assert().success();
}

#[test]
fn test_get_with_timeout() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("get")
        .arg("https://example.com")
        .arg("--timeout")
        .arg("30")
        .arg("--quiet");
    cmd.assert().success();
}

#[test]
fn test_post_with_data() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("post")
        .arg("https://example.com")
        .arg("-d")
        .arg(r#"{"key":"value"}"#)
        .arg("--quiet");
    cmd.assert().success();
}

#[test]
fn test_post_with_all_options() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("post")
        .arg("https://example.com")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(r#"{"key":"value"}"#)
        .arg("-u")
        .arg("user:password")
        .arg("--timeout")
        .arg("30")
        .arg("--quiet");
    cmd.assert().success();
}

#[test]
fn test_put_with_data() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("put")
        .arg("https://example.com")
        .arg("-d")
        .arg(r#"{"key":"value"}"#)
        .arg("--quiet");
    cmd.assert().success();
}

#[test]
fn test_delete_basic() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("delete").arg("https://example.com").arg("--quiet");
    cmd.assert().success();
}

#[test]
fn test_patch_with_data() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("patch")
        .arg("https://example.com")
        .arg("-d")
        .arg(r#"{"key":"value"}"#)
        .arg("--quiet");
    cmd.assert().success();
}

#[test]
fn test_head_basic() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("head").arg("https://example.com").arg("--quiet");
    cmd.assert().success();
}

#[test]
fn test_options_basic() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("options").arg("https://example.com").arg("--quiet");
    cmd.assert().success();
}

#[test]
fn test_get_with_output_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("response.txt");

    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("get")
        .arg("https://example.com")
        .arg("-o")
        .arg(&output_path)
        .arg("--quiet");
    cmd.assert().success();
}

#[test]
fn test_multiple_headers() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("post")
        .arg("https://example.com")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-H")
        .arg("Accept: application/json")
        .arg("-H")
        .arg("Authorization: Bearer token")
        .arg("--quiet");
    cmd.assert().success();
}

#[test]
fn test_get_with_verbose() {
    let mut cmd = Command::cargo_bin("hurl").unwrap();
    cmd.arg("--verbose")
        .arg("get")
        .arg("https://example.com")
        .arg("--quiet");
    cmd.assert().success();
}
