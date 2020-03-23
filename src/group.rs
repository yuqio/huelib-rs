use crate::{Alert, CoordinateModifierType, Effect, ModifierType};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// A group of lights.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Group {
    /// Identifier of the group.
    #[serde(skip_deserializing)]
    pub id: String,
    /// Name of the group.
    pub name: String,
    /// Identifier of lights that are in this group.
    pub lights: Vec<String>,
    /// Type of the group.
    #[serde(rename(deserialize = "type"))]
    pub kind: Type,
    /// Class of the group. Only used if `group_type` is `Room`.
    pub class: Option<Class>,
    /// State of the group.
    pub state: Option<State>,
    /// Uniquely identifies the hardware model of the luminaire. Only present for automatically
    /// created Luminaires.
    #[serde(rename(deserialize = "modelid"))]
    pub model_id: Option<String>,
    /// Unique Id in AA:BB:CC:DD format for Luminaire groups or AA:BB:CC:DD-XX format for
    /// Lightsource groups, where XX is the lightsource position.
    #[serde(rename(deserialize = "unique_id"))]
    pub unique_id: Option<String>,
    // TODO: Sensors
}

impl Group {
    pub(crate) fn with_id(self, id: &str) -> Self {
        Self {
            id: id.to_owned(),
            ..self
        }
    }
}

/// Type of a group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Type {
    /// A special group containing all lights in the system, and is not returned by the
    /// `get_all_groups` function. This group is not visible, and cannot be created, modified or
    /// deleted using the API.
    Zero,
    /// A lighting installation of default groupings of hue lights. The bridge will pre-install
    /// these groups for ease of use. This type cannot be created manually.  Also, a light can only
    /// be in a maximum of one luminaire group. See multisource luminaires for more info.
    Luminaire,
    /// A group of lights which is created by the bridge based on multisource luminaire attributes
    /// of Zigbee light resource.
    #[serde(rename(deserialize = "Lightsource"))]
    LightSource,
    /// A group of lights that can be controlled together. This the default group type that the
    /// bridge generates for user created groups. Default type when no type is given on creation.
    LightGroup,
    /// A group of lights that are physically located in the same place in the house. Rooms behave
    /// similar as light groups, except: (1) A room can be empty and contain 0 lights, (2) a light
    /// is only allowed in one room and (3) a room isn not automatically deleted when all lights in
    /// that room are deleted.
    Room,
    /// A group of lights that are used in an entertainment setup. Entertainment group behave in a
    /// similar way as light groups, with the exception: it can be empty and contain 0 lights. The
    /// group is also not automatically recycled when lights are deleted. The group of lights can
    /// be controlled together as in LightGroup.
    Entertainment,
    /// Zones describe a group of lights that can be controlled together. Zones can be empty and
    /// contain 0 lights. A light is allowed to be in multiple zones.
    Zone,
}

/// Class of a group.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Class {
    Attic,
    Balcony,
    Barbecue,
    Bathroom,
    Bedroom,
    Carport,
    Closet,
    Computer,
    Dining,
    Downstairs,
    Driveway,
    #[serde(rename(deserialize = "Front door"))]
    FrontDoor,
    Garage,
    Garden,
    #[serde(rename(deserialize = "Guest room"))]
    GuestRoom,
    Gym,
    Hallway,
    Home,
    #[serde(rename(deserialize = "Kids bedroom"))]
    KidsBedroom,
    Kitchen,
    #[serde(rename(deserialize = "Laundry room"))]
    LaundryRoom,
    #[serde(rename(deserialize = "Living room"))]
    LivingRoom,
    Lounge,
    #[serde(rename(deserialize = "Man cave"))]
    ManCave,
    Music,
    Nursery,
    Office,
    Other,
    Pool,
    Porch,
    Reading,
    Recreation,
    Staircase,
    Storage,
    Studio,
    TV,
    Terrace,
    Toilet,
    #[serde(rename(deserialize = "Top floor"))]
    TopFloor,
    Upstairs,
}

/// State of a group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct State {
    /// Whether any light in a group is on.
    pub any_on: bool,
    /// Whether all lights in a group are on.
    pub all_on: bool,
}

/// Struct for creating a new group.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Creator {
    /// The name of the new group.
    pub name: String,
    /// Identifier of the lights that will be in the new group.
    pub lights: Vec<String>,
    /// The type of the new group.
    #[serde(skip_serializing_if = "Option::is_none", rename(deserialize = "type"))]
    pub kind: Option<TypeCreator>,
    /// The class of the new group. It is only used when `group_type` is `Room`. If `group_type` is
    /// `Room` and `class` is `None` the room will get the class `Other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<Class>,
}

/// Group type of a `Creator`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum TypeCreator {
    /// A group of lights that can be controlled together. This the default group type that the
    /// bridge generates for user created groups. Default type when no type is given on creation.
    LightGroup,
    /// A group of lights that are physically located in the same place in the house. Rooms behave
    /// similar as light groups, except: (1) A room can be empty and contain 0 lights, (2) a light
    /// is only allowed in one room and (3) a room isn not automatically deleted when all lights in
    /// that room are deleted.
    Room,
    /// A group of lights that are used in an entertainment setup. Entertainment group behave in a
    /// similar way as light groups, with the exception: it can be empty and contain 0 lights. The
    /// group is also not automatically recycled when lights are deleted. The group of lights can
    /// be controlled together as in LightGroup.
    Entertainment,
    /// Zones describe a group of lights that can be controlled together. Zones can be empty and
    /// contain 0 lights. A light is allowed to be in multiple zones.
    Zone,
}

/// Struct for modifying group attributes.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct AttributeModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lights: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    class: Option<Class>,
}

impl AttributeModifier {
    /// Creates a new attribute modifier.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Whether all attributes are `None`.
    pub fn is_empty(&self) -> bool {
        self.name == None && self.lights == None && self.class == None
    }

    /// Changes the name of the group.
    pub fn name(self, value: &str) -> Self {
        Self {
            name: Some(value.to_owned()),
            ..self
        }
    }

    /// Changes what lights are in the group.
    pub fn lights(self, value: &[&str]) -> Self {
        Self {
            lights: Some(Vec::from(value).iter().map(|v| (*v).to_string()).collect()),
            ..self
        }
    }

    /// Changes the class of the group.
    pub fn class(self, value: Class) -> Self {
        Self {
            class: Some(value),
            ..self
        }
    }
}

/// Struct for modifying the group state.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct StateModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bri")]
    brightness: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hue: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sat")]
    saturation: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xy")]
    color_space_coordinates: Option<(f32, f32)>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ct")]
    color_temperature: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alert: Option<Alert>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<Effect>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "transitiontime")]
    transition_time: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bri_inc")]
    brightness_increment: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hue_inc")]
    hue_increment: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sat_inc")]
    saturation_increment: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xy_inc")]
    color_space_coordinates_increment: Option<(f32, f32)>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ct_inc")]
    color_temperature_increment: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scene: Option<String>,
}

impl StateModifier {
    /// Creates a new state modifier.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Whether all attributes are `None`.
    pub fn is_empty(&self) -> bool {
        self.on == None
            && self.brightness == None
            && self.hue == None
            && self.saturation == None
            && self.color_space_coordinates == None
            && self.color_temperature == None
            && self.alert == None
            && self.effect == None
            && self.transition_time == None
            && self.brightness_increment == None
            && self.hue_increment == None
            && self.saturation_increment == None
            && self.color_space_coordinates_increment == None
            && self.color_temperature_increment == None
            && self.scene == None
    }

    /// Turns the lights on or off.
    pub fn on(self, value: bool) -> Self {
        Self {
            on: Some(value),
            ..self
        }
    }

    /// Sets the brightness of the lights.
    pub fn brightness(self, modifier_type: ModifierType, value: u8) -> Self {
        match modifier_type {
            ModifierType::Override => Self {
                brightness: Some(value),
                ..self
            },
            ModifierType::Increment => Self {
                brightness_increment: Some(value as i16),
                ..self
            },
            ModifierType::Decrement => Self {
                brightness_increment: Some(-(value as i16)),
                ..self
            },
        }
    }

    /// Sets the hue of the lights.
    pub fn hue(self, modifier_type: ModifierType, value: u16) -> Self {
        match modifier_type {
            ModifierType::Override => Self {
                hue: Some(value),
                ..self
            },
            ModifierType::Increment => Self {
                hue_increment: Some(value as i32),
                ..self
            },
            ModifierType::Decrement => Self {
                hue_increment: Some(-(value as i32)),
                ..self
            },
        }
    }

    /// Sets the saturation of the lights.
    pub fn saturation(self, modifier_type: ModifierType, value: u8) -> Self {
        match modifier_type {
            ModifierType::Override => Self {
                saturation: Some(value),
                ..self
            },
            ModifierType::Increment => Self {
                saturation_increment: Some(value as i16),
                ..self
            },
            ModifierType::Decrement => Self {
                saturation_increment: Some(-(value as i16)),
                ..self
            },
        }
    }

    /// Sets the x and y coordinates in the color space to set the color of the lights.
    ///
    /// If the modifier type is `Override`, the values must be between 0 and 1. If the modifier
    /// type is not `Override`, the values must be between 0 and 0.5.
    pub fn color_space_coordinates(
        self,
        modifier_type: CoordinateModifierType,
        value: (f32, f32),
    ) -> Self {
        match modifier_type {
            CoordinateModifierType::Override => Self {
                color_space_coordinates: Some(value),
                ..self
            },
            CoordinateModifierType::Increment => Self {
                color_space_coordinates_increment: Some(value),
                ..self
            },
            CoordinateModifierType::Decrement => Self {
                color_space_coordinates_increment: Some((-value.0, -value.1)),
                ..self
            },
            CoordinateModifierType::IncrementDecrement => Self {
                color_space_coordinates_increment: Some((value.0, -value.1)),
                ..self
            },
            CoordinateModifierType::DecrementIncrement => Self {
                color_space_coordinates_increment: Some((-value.0, value.1)),
                ..self
            },
        }
    }

    /// Sets the color temperature of the lights.
    pub fn color_temperature(self, modifier_type: ModifierType, value: u16) -> Self {
        match modifier_type {
            ModifierType::Override => Self {
                color_temperature: Some(value),
                ..self
            },
            ModifierType::Increment => Self {
                color_temperature_increment: Some(value as i32),
                ..self
            },
            ModifierType::Decrement => Self {
                color_temperature_increment: Some(-(value as i32)),
                ..self
            },
        }
    }

    /// Sets the alert effect of the lights.
    pub fn alert(self, value: Alert) -> Self {
        Self {
            alert: Some(value),
            ..self
        }
    }

    /// Sets the dynamic effect of the lights.
    pub fn effect(self, value: Effect) -> Self {
        Self {
            effect: Some(value),
            ..self
        }
    }

    /// Sets the duration of the transition from the light's current state to the new state. This
    /// is given as a multiple of 100ms.
    pub fn transition_time(self, value: u16) -> Self {
        Self {
            transition_time: Some(value),
            ..self
        }
    }

    /// Sets the scene of the group.
    pub fn scene(self, value: &str) -> Self {
        Self {
            scene: Some(value.to_owned()),
            ..self
        }
    }
}