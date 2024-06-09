
use crate::build::props::Props;

pub trait PropsConnect: Props {
    type PropsConnector;
}
