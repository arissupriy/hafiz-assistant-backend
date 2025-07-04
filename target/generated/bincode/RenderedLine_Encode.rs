impl :: bincode :: Encode for RenderedLine
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.text, encoder) ?; :: bincode ::
        Encode :: encode(&self.line_type, encoder) ?; :: bincode :: Encode ::
        encode(&self.is_centered, encoder) ?; core :: result :: Result ::
        Ok(())
    }
}