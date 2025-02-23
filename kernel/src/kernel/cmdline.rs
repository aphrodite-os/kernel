//! Provide functions and structs for parsing a kernel command line.

/// A value of an argument.
#[derive(Clone, Copy)]
pub enum ArgumentValue {
    /// A string argument with the preceeding and following single quotes
    /// removed, and any \' replaced with '.
    Str(&'static str),
    /// A float argument.
    Float(f128),
    /// A signed argument.
    Signed(i128),
    /// A unsigned argument.
    Unsigned(u128),
}

/// A single argument in a [Cmdline].
#[derive(Clone, Copy)]
pub struct Argument {
    /// The name of an argument.
    pub name: &'static str,
    /// The value of an argument.
    pub value: ArgumentValue,
}

/// A single flag in a [Cmdline].
#[derive(Clone, Copy)]
pub struct Flag {
    /// The name of a flag.
    pub name: &'static str,
}

/// A kernel command line.
#[derive(Clone)]
pub struct Cmdline {
    /// The arguments of the Cmdline.
    pub arguments: &'static [Argument],
    /// The flags of the Cmdline.
    pub flags: &'static [Flag],

    /// The argument validators. When using [CmdlineValidator], it will check
    /// all of them and if ALL of them report ANY of the arguments
    /// incorrect, then it will return an error.
    pub argument_validators: &'static [&'static dyn ArgumentValidator],

    /// The flag validators. When using [CmdlineValidator], it will check all of
    /// them and if ALL of them report ANY of the flags incorrect, then it
    /// will return an error.
    pub flag_validators: &'static [&'static dyn FlagValidator],
}

/// A validator of one of the types in this module.
pub trait Validator {
    /// The type that the trait validates.
    type Validates;

    /// Validate a value.
    fn validate<'a>(&self, value: Self::Validates) -> Result<(), crate::Error<'a>>;
}

/// A [Validator] that validates arguments.
pub trait ArgumentValidator: Validator<Validates = Argument> {}

/// A [Validator] that validates flags.
pub trait FlagValidator: Validator<Validates = Flag> {}

/// A [Validator] that validates cmdlines.
pub struct CmdlineValidator {}

/// Error returned by [CmdlineValidator::validate] when an argument is incorrect
pub const ERR_INVALID_ARGUMENT: i16 = -1;

/// Error returned by [CmdlineValidator::validate] when a flag is incorrect
pub const ERR_INVALID_FLAG: i16 = -2;

impl Validator for CmdlineValidator {
    type Validates = Cmdline;

    fn validate<'a>(&self, value: Self::Validates) -> Result<(), crate::Error<'a>> {
        for arg in value.arguments {
            let mut correct = false;
            for validator in value.argument_validators {
                if validator.validate(*arg).is_ok() {
                    correct = true;
                    break;
                }
            }
            if !correct {
                return Err(crate::Error::new(
                    "invalid argument in command line",
                    ERR_INVALID_ARGUMENT,
                ));
            }
        }

        for arg in value.flags {
            let mut correct = false;
            for validator in value.flag_validators {
                if validator.validate(*arg).is_ok() {
                    correct = true;
                    break;
                }
            }
            if !correct {
                return Err(crate::Error::new(
                    "invalid flag in command line",
                    ERR_INVALID_FLAG,
                ));
            }
        }
        Ok(())
    }
}
