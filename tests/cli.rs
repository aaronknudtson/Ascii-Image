use std::{process::Command, env::current_dir};
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;



#[test]
fn fails_on_fake_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("asciifire")?
        .arg("/test/doesnt/exist")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to open image at provided path"));
    Ok(())
}

#[test]
fn is_successful() -> Result<(), Box<dyn std::error::Error>> {
    let path = current_dir()?;
    Command::cargo_bin("asciifire")?
        .arg(path.join("Savior.jpeg"))
        .assert()
        .success();
    Ok(())
}

#[test]
fn height_only() -> Result<(), Box<dyn std::error::Error>> {
    let height = 1000;
    let path = current_dir()?;
    let temp = assert_fs::TempDir::new()?;
    let input_file = temp.child("foo.txt");
    input_file.touch()?;

    Command::cargo_bin("asciifire")?
        .arg(path.join("Savior.jpeg"))
        .arg("-o").arg(input_file.path())
        .arg("--height").arg(height.to_string())
        .assert()
        .success();

    let actual = std::fs::read_to_string(input_file.path())?.lines().count();
    temp.close()?; 

    assert_eq!(height, actual);
    Ok(())
}

#[test]
fn width_only() -> Result<(), Box<dyn std::error::Error>> {
    let width = 1000;
    let path = current_dir()?;
    let temp = assert_fs::TempDir::new()?;
    let input_file = temp.child("foo.txt");
    input_file.touch()?;

    Command::cargo_bin("asciifire")?
        .arg(path.join("Savior.jpeg"))
        .arg("-o").arg(input_file.path())
        .arg("--width").arg(width.to_string())
        .assert()
        .success();

    let actual = std::fs::read_to_string(input_file.path())?.lines().last().unwrap().len();
    temp.close()?; 

    assert_eq!(width, actual);
    Ok(())
}

#[test]
fn height_and_width() -> Result<(), Box<dyn std::error::Error>> {
    let width = 1000;
    let height = 1000;
    let path = current_dir()?;
    let temp = assert_fs::TempDir::new()?;
    let input_file = temp.child("foo.txt");
    input_file.touch()?;

    Command::cargo_bin("asciifire")?
        .arg(path.join("Savior.jpeg"))
        .arg("-o").arg(input_file.path())
        .arg("--width").arg(width.to_string())
        .arg("--height").arg(height.to_string())
        .assert()
        .success();

    let contents = std::fs::read_to_string(input_file.path())?;
    let actual_width = contents.lines().last().unwrap().len();
    let actual_height = contents.lines().count();
    temp.close()?; 

    assert_eq!(width, actual_width);
    assert_eq!(height, actual_height);
    Ok(())
}

#[test]
fn output_only() -> Result<(), Box<dyn std::error::Error>> {
    let path = current_dir()?;
    let temp = assert_fs::TempDir::new()?;
    let input_file = temp.child("foo.txt");
    input_file.touch()?;

    Command::cargo_bin("asciifire")?
        .arg(path.join("Savior.jpeg"))
        .arg("-o").arg(input_file.path())
        .assert()
        .success();

    let img = image::open(path.join("Savior.jpeg"))?;
    let contents = std::fs::read_to_string(input_file.path())?;
    let actual_width = contents.lines().last().unwrap().len();
    let actual_height = contents.lines().count();
    assert_eq!(actual_height, img.height() as usize);
    assert_eq!(actual_width, img.width() as usize * 2);

    Ok(())
}
