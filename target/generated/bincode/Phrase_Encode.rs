impl :: bincode :: Encode for Phrase
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.surahs, encoder) ?; :: bincode ::
        Encode :: encode(&self.ayahs, encoder) ?; :: bincode :: Encode ::
        encode(&self.count, encoder) ?; :: bincode :: Encode ::
        encode(&self.source, encoder) ?; core :: result :: Result :: Ok(())
    }
}