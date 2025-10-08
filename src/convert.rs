use std::fmt::{Display, Formatter};

use clap::builder::PossibleValue;
use clap::ValueEnum;
use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase,
    ToShoutySnakeCase, ToSnakeCase, ToTrainCase, ToTitleCase
};

/// [clap::ValueEnum] to convert input string to a target case
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum ToCase {
    KebabCase,
    CamelCase,
    PascalCase,
    ShoutyKebabCase,
    ShoutySnakeCase,
    SnakeCase,
    TrainCase,
    TitleCase,
}
impl Display for ToCase {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let variant = self.variant_name().to_kebab_case();
        write!(f, "{variant}")
    }
}
impl ToCase {
    pub fn variant_name(&self) -> &'static str {
        match self {
            ToCase::KebabCase => "Kebab",
            ToCase::CamelCase => "Camel",
            ToCase::PascalCase => "Pascal",
            ToCase::ShoutyKebabCase => "ShoutyKebab",
            ToCase::ShoutySnakeCase => "ShoutySnake",
            ToCase::SnakeCase => "Snake",
            ToCase::TrainCase => "Train",
            ToCase::TitleCase => "Title",
        }
    }

    pub fn convert<T: Display>(&self, input: T) -> String {
        let string = input.to_string();
        match self {
            ToCase::KebabCase => string.to_kebab_case(),
            ToCase::CamelCase => string.to_lower_camel_case(),
            ToCase::PascalCase => string.to_pascal_case(),
            ToCase::ShoutyKebabCase => string.to_shouty_kebab_case(),
            ToCase::ShoutySnakeCase => string.to_shouty_snake_case(),
            ToCase::SnakeCase => string.to_snake_case(),
            ToCase::TrainCase => string.to_train_case(),
            ToCase::TitleCase => string.to_title_case(),
        }
    }

    pub fn variant_name_with_to_prefix(&self) -> String {
        let variant = self.variant_name();
        format!("To{variant}")
    }

    pub fn variant_name_with_case_suffix(&self) -> String {
        let variant = self.variant_name();
        format!("{variant}Case")
    }

    pub fn variant_name_with_prefix_and_suffix(&self) -> String {
        let variant = self.variant_name();
        format!("To{variant}Case")
    }

    pub fn base_variant_names(&self) -> [String; 4] {
        [
            self.variant_name().to_string(),
            self.variant_name_with_to_prefix(),
            self.variant_name_with_case_suffix(),
            self.variant_name_with_prefix_and_suffix(),
        ]
    }

    pub fn variant_names(&self) -> Vec<String> {
        let mut variants = Vec::<String>::new();

        for variant in self
            .base_variant_names()
            .into_iter()
            .map(|variant| {
                [
                    variant.to_kebab_case(),
                    variant.to_lower_camel_case(),
                    variant.to_pascal_case(),
                    variant.to_train_case(),
                    variant.to_title_case(),
                    variant.to_snake_case(),
                    variant.to_shouty_snake_case(),
                    variant.to_shouty_kebab_case(),
                ]
                .to_vec()
            })
            .flatten()
        {
            if !variants.contains(&variant) {
                variants.push(variant);
            }
        }
        variants
    }

    pub fn variants<'a>() -> &'a [ToCase] {
        &[
            ToCase::KebabCase,
            ToCase::CamelCase,
            ToCase::PascalCase,
            ToCase::ShoutyKebabCase,
            ToCase::ShoutySnakeCase,
            ToCase::SnakeCase,
            ToCase::TrainCase,
            ToCase::TitleCase,
        ]
    }
}
impl ValueEnum for ToCase {
    fn value_variants<'a>() -> &'a [ToCase] {
        ToCase::variants()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        let variants = self.variant_names();
        let mut pv = PossibleValue::new(variants[0].to_string());
        for variant in &variants[1..] {
            pv = pv.alias(variant);
        }
        Some(pv)
    }

    fn from_str(
        val: &str,
        ignore_case: bool,
    ) -> std::result::Result<ToCase, String> {
        let val = if ignore_case {
            val.to_lowercase()
        } else {
            val.to_string()
        };
        let val = val.trim();
        for variant in ToCase::variants() {
            for variant_name in
                variant
                    .variant_names()
                    .into_iter()
                    .map(|variant| {
                        if ignore_case {
                            variant.to_lowercase().to_string()
                        } else {
                            variant.to_string()
                        }
                    })
            {
                if val.to_string() == variant_name {
                    return Ok(variant.clone());
                }
            }
        }
        return Err(val.to_string());
    }
}
