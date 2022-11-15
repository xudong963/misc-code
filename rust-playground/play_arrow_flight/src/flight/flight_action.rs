use arrow_format::flight::data::Action;
use tonic::Status;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum FlightAction {
    Null,
    Action1,
    Action2,
    Action3,
}

impl TryInto<Action> for FlightAction {
    type Error = Status;

    fn try_into(self) -> Result<Action, Self::Error> {
        match self {
            FlightAction::Null => Ok(Action {
                r#type: "null".to_string(),
                body: vec![],
            }),
            FlightAction::Action1 => Ok(Action {
                r#type: "action1".to_string(),
                body: vec![1],
            }),
            FlightAction::Action2 => Ok(Action {
                r#type: "action2".to_string(),
                body: vec![2],
            }),
            FlightAction::Action3 => Ok(Action {
                r#type: "action3".to_string(),
                body: vec![3],
            }),
        }
    }
}

impl TryInto<FlightAction> for Action {
    type Error = Status;

    fn try_into(self) -> Result<FlightAction, Self::Error> {
        match self.r#type.as_str() {
            "action1" => Ok(FlightAction::Action1),
            "action2" => Ok(FlightAction::Action2),
            "action3" => Ok(FlightAction::Action3),
            _ => Err(Status::invalid_argument("Unknown action")),
        }
    }
}
