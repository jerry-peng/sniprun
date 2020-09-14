//Interpreter:| Lua_nvim            | lua (nvim)  |
//############|_____________________|_____________|________________<- delimiters to help formatting,
//###########| Interpretername      | language    | comment
// Keep (but modify the first line after the :) if you wish to have this interpreter listedvia SnipList
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Lua_nvim {
    support_level: SupportLevel,
    data: DataHolder,
    code: String,
    lua_work_dir: String,
    main_file_path: String,
}

impl Interpreter for Lua_nvim {
    fn new_with_level(data: DataHolder, level: SupportLevel) -> Box<Lua_nvim> {
        let bwd = data.work_dir.clone() + "/nvim-lua";
        let mut builder = DirBuilder::new();
        builder.recursive(true);
        builder
            .create(&bwd)
            .expect("Could not create directory for lua-nvim");
        let mfp = bwd.clone() + "/main.lua";
        Box::new(Lua_nvim {
            data,
            support_level: level,
            code: String::from(""),
            lua_work_dir: bwd,
            main_file_path: mfp,
        })
    }

    fn get_name() -> String {
        String::from("lua_nvim")
    }

    fn get_supported_languages() -> Vec<String> {
        vec![String::from("lua")]
    }

    fn get_current_level(&self) -> SupportLevel {
        self.support_level
    }
    fn set_current_level(&mut self, level: SupportLevel) {
        self.support_level = level;
    }

    fn get_data(&self) -> DataHolder {
        self.data.clone()
    }

    fn get_max_support_level() -> SupportLevel {
        SupportLevel::Bloc
    }

    fn fallback(&mut self) -> Option<Result<String, SniprunError>> {
        self.fetch_code().expect("could not fetch code");
        if !(self.code.contains("nvim")) {
            //then this is not lua_nvim code but pure lua one
            let mut good_interpreter = crate::interpreters::Lua_original::new_with_level(
                self.data.clone(),
                self.get_current_level(),
            );
            return Some(good_interpreter.run());
        }
        return None;
    }

    fn fetch_code(&mut self) -> Result<(), SniprunError> {
        if !self
            .data
            .current_bloc
            .replace(&[' ', '\t', '\n', '\r'][..], "")
            .is_empty()
            && self.get_current_level() >= SupportLevel::Bloc
        {
            self.code = self.data.current_bloc.clone();
        } else if !self.data.current_line.replace(" ", "").is_empty()
            && self.get_current_level() >= SupportLevel::Line
        {
            self.code = self.data.current_line.clone();
        } else {
            self.code = String::from("");
        }
        Ok(())
    }

    fn add_boilerplate(&mut self) -> Result<(), SniprunError> {
        Ok(())
    }

    fn build(&mut self) -> Result<(), SniprunError> {
        let mut _file =
            File::create(&self.main_file_path).expect("Failed to create file for lua-nvim");

        write(&self.main_file_path, &self.code).expect("Unable to write to file for lua-nvim");
        Ok(())
    }

    fn execute(&mut self) -> Result<String, SniprunError> {
        let output = Command::new("nvim")
            .arg("--headless")
            .arg("-c")
            .arg(format!("luafile {}", &self.main_file_path))
            .arg("-c")
            .arg("q!")
            .output()
            .expect("Unable to start process");
        info!("yay from lua interpreter");
        if output.status.success() {
            return Ok(String::from_utf8(output.stderr).unwrap());
        } else {
            return Err(SniprunError::RuntimeError(
                String::from_utf8(output.stderr).unwrap(),
            ));
        }
    }
}