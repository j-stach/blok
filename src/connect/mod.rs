

pub trait Connectable {

    type ConnectionInstructions;
    type InterConnector: Fn(&mut Self, &mut Self, &Self::ConnectionInstructions) -> Result<(), anyhow::Error>;
    type AutoConnector: Fn(&mut Self, &Self::ConnectionInstructions) -> Result<(), anyhow::Error>;


    fn connect(
        &mut self,
        other: &mut Self,
        connector: Self::InterConnector,
        instructions: &Self::ConnectionInstructions
    ) -> Result<(), anyhow::Error> {
        connector(self, other, instructions)
    }


}
