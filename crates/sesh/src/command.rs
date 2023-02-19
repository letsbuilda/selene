use miette::Result;

#[derive(Debug)]
pub struct ShellCmd {
    name: String,
    usage: String,
    description: String,

    builtin: bool,
    alias: bool,
}

impl ShellCmd {
    pub fn new(name: String, usage: String, description: String) -> Self {
        Self { name, usage, description, builtin: false, alias: false }
    }

    /// Mark the command as being a builtin.
    pub fn builtin(&mut self) -> &mut Self {
        self.builtin = true;
        self
    }

    /// Mark the command as being an alias.
    pub fn alias(&mut self) -> &mut Self {
        self.alias = true;
        self
    }

    pub fn is_builtin(&self) -> bool {
        self.builtin
    }

    pub fn is_alias(&self) -> bool {
        self.alias
    }

    /// Execute a shell command.
    ///
    /// How it works:
    ///
    /// 1. Parse the shell command.
    /// 2. Check if the command is an alias.
    /// 3. Find the location of the executable program by searching in PATH
    pub fn exec(&mut self) -> Result<()> {
        todo!();
    }
}
