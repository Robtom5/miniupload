use crate::conf;
use anyhow::Result;
use reqwest::blocking::multipart;
use std::env;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

pub fn create_active_folder() -> Result<()> {
    let cfg = conf::ToolConfig::from_app(crate::APP_NAME)?;
    let folder = cfg.get_folder()?;

    match folder.chars().last() {
        Some(_) => {
            let client = reqwest::blocking::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()?;
            let form = multipart::Form::new().text("mkdir", folder.clone());
            client
                .post(cfg.get_upload_target()?)
                .multipart(form)
                .send()?;
            Ok(())
        }
        None => Ok(()),
    }
}

pub fn upload_file(file: PathBuf) -> Result<()> {
    let cfg = conf::ToolConfig::from_app(crate::APP_NAME)?;
    let mut file_path = env::current_dir()?;
    file_path.push(file);

    match file_path.try_exists() {
        Ok(true) => {}
        Ok(false) => panic!("Error evaluating file existence"),
        Err(_) => panic!("Could not find file to upload"),
    }

    create_active_folder()?;

    let form =
        multipart::Form::new().file("path", file_path.into_os_string().into_string().unwrap())?;
    let mut target = cfg.get_upload_target()?;
    let folder = cfg.get_folder()?;

    match folder.chars().last() {
        Some(_) => {
            target.push_str(&folder);
            target.push_str("/");
        }
        None => {}
    }

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    #[cfg(not(debug_assertions))]
    client.post(target).multipart(form).send()?;

    #[cfg(debug_assertions)]
    println!("{}", client.post(target).multipart(form).send()?.text()?);

    Ok(())
}

pub fn download(file: &str, dest: &str) -> Result<()> {
    let mut file_dest = env::current_dir()?;
    file_dest.push(dest.clone());

    let cfg = conf::ToolConfig::from_app(crate::APP_NAME)?;
    let mut src = cfg.get_target()?;
    let folder = cfg.get_folder()?;

    match folder.chars().last() {
        Some(_) => {
            src.push_str(&folder);
            src.push_str("/");
        }
        None => {}
    }

    src.push_str(file);

    let mut dest = File::create(&file_dest)?;

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let mut content = client.get(src).send()?;

    copy(&mut content, &mut dest)?;

    Ok(())
}
