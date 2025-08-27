use std::fs;
use tempfile::TempDir;
use assert_cmd::Command;
use predicates::prelude::*;

/// 測試完整的工作流程: init -> build -> deploy -> test
#[test]
fn test_complete_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let project_name = "integration-test";
    
    // 1. 測試 init
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("init")
        .arg(project_name)
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Successfully created"));
    
    let project_dir = temp_dir.path().join(project_name);
    assert!(project_dir.exists());
    assert!(project_dir.join("qanchor.yaml").exists());
    assert!(project_dir.join("src/oracle.qidl").exists());
    
    // 2. 測試 build
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("build")
        .current_dir(&project_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Command completed successfully"));
    
    assert!(project_dir.join("target/debug/contract.wasm").exists());
    assert!(project_dir.join("target/qidl/contract.json").exists());
    
    // 3. 測試 deploy
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("deploy")
        .arg("--network")
        .arg("local")
        .arg("--yes")
        .current_dir(&project_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Contract deployed successfully"));
    
    // 4. 測試 test
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("test")
        .current_dir(&project_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("tests passed"));
}

/// 測試 SDK 生成功能
#[test]
fn test_sdk_generation() {
    let temp_dir = TempDir::new().unwrap();
    let project_name = "sdk-test";
    
    // 建立測試專案
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("init")
        .arg(project_name)
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    let project_dir = temp_dir.path().join(project_name);
    
    // 測試 TypeScript SDK 生成
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("generate")
        .arg("--lang")
        .arg("ts")
        .arg("--output")
        .arg("./ts-sdk")
        .current_dir(&project_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("SDK Generated Successfully"));
    
    // 驗證生成的檔案
    let ts_dir = project_dir.join("ts-sdk");
    assert!(ts_dir.join("types.ts").exists());
    assert!(ts_dir.join("client.ts").exists());
    assert!(ts_dir.join("index.ts").exists());
    assert!(ts_dir.join("package.json").exists());
    
    // 驗證 TypeScript 內容
    let types_content = fs::read_to_string(ts_dir.join("types.ts")).unwrap();
    assert!(types_content.contains("export interface"));
    assert!(types_content.contains("PriceData"));
    
    // 測試 Python SDK 生成
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("generate")
        .arg("--lang")
        .arg("py")
        .arg("--output")
        .arg("./py-sdk")
        .current_dir(&project_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("SDK Generated Successfully"));
    
    // 驗證生成的檔案
    let py_dir = project_dir.join("py-sdk");
    assert!(py_dir.join("types.py").exists());
    assert!(py_dir.join("client.py").exists());
    assert!(py_dir.join("requirements.txt").exists());
    assert!(py_dir.join("__init__.py").exists());
    
    // 驗證 Python 內容
    let types_content = fs::read_to_string(py_dir.join("types.py")).unwrap();
    assert!(types_content.contains("@dataclass"));
    assert!(types_content.contains("PriceData"));
}

/// 測試錯誤情況
#[test]
fn test_error_scenarios() {
    let temp_dir = TempDir::new().unwrap();
    
    // 測試在非專案目錄執行 build
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("build")
        .current_dir(&temp_dir)
        .assert()
        .failure()
        .stderr(predicate::str::contains("No qanchor.yaml found"));
    
    // 測試 deploy 沒有先 build
    let project_name = "error-test";
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("init")
        .arg(project_name)
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    let project_dir = temp_dir.path().join(project_name);
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("deploy")
        .arg("--network")
        .arg("local")
        .arg("--yes")
        .current_dir(&project_dir)
        .assert()
        .failure()
        .stderr(predicate::str::contains("No build artifacts found"));
    
    // 測試無效的網路名稱
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("build")
        .current_dir(&project_dir)
        .assert()
        .success();
    
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("deploy")
        .arg("--network")
        .arg("invalid")
        .arg("--yes")
        .current_dir(&project_dir)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown network"));
}

/// 測試 clean 功能
#[test]
fn test_clean_functionality() {
    let temp_dir = TempDir::new().unwrap();
    let project_name = "clean-test";
    
    // 建立專案並產生檔案
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("init")
        .arg(project_name)
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    let project_dir = temp_dir.path().join(project_name);
    
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("build")
        .current_dir(&project_dir)
        .assert()
        .success();
    
    // 確認檔案存在
    assert!(project_dir.join("target").exists());
    
    // 測試 clean
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("clean")
        .current_dir(&project_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Cleaned"));
    
    // 確認檔案被清理
    assert!(!project_dir.join("target").exists());
}

/// 測試 generate 指令的錯誤情況
#[test]
fn test_generate_error_cases() {
    let temp_dir = TempDir::new().unwrap();
    
    // 測試不存在的輸入檔案
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("generate")
        .arg("--lang")
        .arg("ts")
        .arg("--input")
        .arg("nonexistent.qidl")
        .current_dir(&temp_dir)
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
    
    // 測試不支援的語言
    let mut cmd = Command::cargo_bin("qanchor").unwrap();
    cmd.arg("generate")
        .arg("--lang")
        .arg("invalid")
        .current_dir(&temp_dir)
        .assert()
        .failure()
        .stderr(predicate::str::contains("QIDL file not found"));
}
