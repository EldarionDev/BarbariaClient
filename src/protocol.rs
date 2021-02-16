#[non_exhaustive]
pub struct Target;

impl Target {
    pub const Application: f32 = 0.0;
    pub const Game: f32 = 1.0;
    pub const Engine: f32 = 2.0;
    pub const Network: f32 = 3.0;
}

#[non_exhaustive]
pub struct ApplicationCommand;

impl ApplicationCommand {
    pub const Exit: f32 = 0.0;
}

#[non_exhaustive]
pub struct GameCommand;

impl GameCommand {
    pub const AddArmy: f32 = 0.0;
    pub const RemoveArmy: f32 = 0.5;
    pub const AddUnit: f32 = 1.0;
    pub const RemoveUnit: f32 = 1.5;
    pub const AlterSettlement: f32 = 2.0;
    pub const AlterFactionStats: f32 = 3.0;
    pub const AddBuilding: f32 = 4.0;
}

#[non_exhaustive]
pub struct EngineCommand;

impl EngineCommand {
    pub const AddGui: f32 = 0.0;
    pub const RemoveGui: f32 = 0.5;
    pub const AddSprite: f32 = 1.0;
    pub const RemoveSprite: f32 = 1.5;
    pub const AddObject: f32 = 2.0;
    pub const RemoveObject: f32 = 2.5;
}

#[non_exhaustive]
pub struct NetworkCommand;

impl NetworkCommand {
    pub const SendData: f32 = 0.0;
}