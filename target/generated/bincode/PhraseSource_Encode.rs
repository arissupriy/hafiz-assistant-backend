impl :: bincode :: Encode for PhraseSource
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.key, encoder) ?; :: bincode ::
        Encode :: encode(&self.from_idx, encoder) ?; :: bincode :: Encode ::
        encode(&self.to_idx, encoder) ?; core :: result :: Result :: Ok(())
    }
}