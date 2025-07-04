impl :: bincode :: Encode for RenderedPage
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.page_number, encoder) ?; ::
        bincode :: Encode :: encode(&self.lines, encoder) ?; core :: result ::
        Result :: Ok(())
    }
}