impl :: bincode :: Encode for TafsirJalalaynEntry
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.text, encoder) ?; core :: result
        :: Result :: Ok(())
    }
}