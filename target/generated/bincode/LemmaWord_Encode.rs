impl :: bincode :: Encode for LemmaWord
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.lemma_id, encoder) ?; :: bincode
        :: Encode :: encode(&self.word_location, encoder) ?; :: bincode ::
        Encode :: encode(&self.text, encoder) ?; core :: result :: Result ::
        Ok(())
    }
}