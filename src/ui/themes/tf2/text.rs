use iced::{
    color,
    widget::text::{Catalog, Style, StyleFn},
};

use super::super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn default(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().surface.text),
        selection: theme.colors().primary.color.scale_alpha(0.5),
    }
}

// TODO: Hardcoded...
pub fn muted(theme: &Theme) -> Style {
    Style {
        color: Some(color!(0x574f47)),
        selection: theme.colors().primary.color.scale_alpha(0.5),
    }
}

pub fn success(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().success.color),
        selection: theme.colors().primary.color.scale_alpha(0.5),
    }
}

pub fn primary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().primary.color),
        selection: theme.colors().primary.color.scale_alpha(0.5),
    }
}

pub fn primary_container(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().primary.container_text),
        selection: theme.colors().primary.color.scale_alpha(0.5),
    }
}

pub fn secondary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().secondary.color),
        selection: theme.colors().primary.color.scale_alpha(0.5),
    }
}

pub fn secondary_container(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().secondary.container_text),
        selection: theme.colors().primary.color.scale_alpha(0.5),
    }
}

pub fn tertiary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().tertiary.text),
        selection: theme.colors().primary.color.scale_alpha(0.5),
    }
}

pub fn tertiary_container(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().tertiary.container_text),
        selection: theme.colors().primary.color.scale_alpha(0.5),
    }
}

pub fn error(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().error.text),
        selection: theme.colors().primary.color.scale_alpha(0.5),
    }
}

pub fn error_container(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().error.container_text),
        selection: theme.colors().primary.color.scale_alpha(0.5),
    }
}

pub fn surface(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().surface.text),
        selection: theme.colors().primary.color.scale_alpha(0.5),
    }
}

pub fn surface_variant(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().surface.text_variant),
        selection: theme.colors().primary.color.scale_alpha(0.5),
    }
}

pub fn inverse_surface(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().inverse.inverse_surface),
        selection: theme.colors().primary.color.scale_alpha(0.5),
    }
}
