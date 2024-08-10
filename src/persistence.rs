use std::{
    collections::HashMap,
    ffi::OsString,
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

/// Map command key to shell command to execute
type DirectoryCommands = HashMap<String, OsString>;

#[derive(Serialize, Deserialize, Default)]
struct SystemCommands {
    dir_commands: HashMap<PathBuf, DirectoryCommands>,
}

#[allow(dead_code)]
impl SystemCommands {
    pub fn new(dir_commands: HashMap<PathBuf, DirectoryCommands>) -> Self {
        Self { dir_commands }
    }

    pub fn get_dir_commands(&self, path: &PathBuf) -> Option<&DirectoryCommands> {
        self.dir_commands.get(path)
    }

    pub fn get_dir_commands_mut(&mut self, path: &PathBuf) -> Option<&mut DirectoryCommands> {
        self.dir_commands.get_mut(path)
    }

    pub fn add_dir_commands(&mut self, path: PathBuf, dir_commands: DirectoryCommands) {
        self.dir_commands.insert(path, dir_commands);
    }

    pub fn remove_dir_commands(&mut self, path: &PathBuf) -> Option<DirectoryCommands> {
        self.dir_commands.remove(path)
    }

    pub fn save(&self, path: &PathBuf) -> io::Result<()> {
        let serialized_data = serde_json::to_string(self)?;

        let mut file = File::create(path)?;
        file.write_all(serialized_data.as_bytes())?;
        Ok(())
    }

    pub fn load(path: &PathBuf) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let commands: Self = serde_json::from_str(&content)?;
        Ok(commands)
    }
}

#[cfg(test)]
mod tests {
    use std::{env::current_dir, io};

    use super::*;

    #[test]
    fn test_get_none() {
        let mut commands = SystemCommands::default();

        assert!(commands.get_dir_commands(&PathBuf::new()).is_none());
        assert!(commands.get_dir_commands_mut(&PathBuf::new()).is_none());
    }

    #[test]
    fn test_add_and_get() -> Result<(), io::Error> {
        let dir_commands =
            DirectoryCommands::from([("run".to_string(), OsString::from("make run"))]);
        let path = current_dir()?;

        let mut commands = SystemCommands::default();
        commands.add_dir_commands(path.clone(), dir_commands);

        assert!(commands.get_dir_commands(&path).is_some());
        assert!(commands.get_dir_commands_mut(&path).is_some());

        Ok(())
    }

    #[test]
    fn test_remove() -> Result<(), io::Error> {
        let dir_commands =
            DirectoryCommands::from([("run".to_string(), OsString::from("make run"))]);
        let path = current_dir()?;

        let mut commands = SystemCommands::new(HashMap::from([(path.clone(), dir_commands)]));

        assert!(commands.get_dir_commands(&path).is_some());
        assert!(commands.get_dir_commands_mut(&path).is_some());

        assert!(commands.remove_dir_commands(&path).is_some());

        assert!(commands.get_dir_commands(&path).is_none());
        assert!(commands.get_dir_commands_mut(&path).is_none());

        Ok(())
    }
}
