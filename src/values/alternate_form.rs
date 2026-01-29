#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlternateForm {
    Deactivated,
    Activated,
}

pub trait ToAlternateForm {
    fn to_alternate_form(self) -> AlternateForm;
}

impl ToAlternateForm for bool {
    fn to_alternate_form(self) -> AlternateForm {
        AlternateForm::of(self)
    }
}

impl ToAlternateForm for AlternateForm {
    fn to_alternate_form(self) -> AlternateForm {
        self
    }
}

impl AlternateForm {
    pub fn of(bool: bool) -> Self {
        match bool {
            false => AlternateForm::Deactivated,
            true => AlternateForm::Activated,
        }
    }
}
