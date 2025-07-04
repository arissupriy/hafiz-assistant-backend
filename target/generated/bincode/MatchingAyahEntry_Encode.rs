impl :: bincode :: Encode for MatchingAyahEntry
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.matched_ayah_key, encoder) ?; ::
        bincode :: Encode :: encode(&self.matched_words_count, encoder) ?; ::
        bincode :: Encode :: encode(&self.coverage, encoder) ?; :: bincode ::
        Encode :: encode(&self.score, encoder) ?; :: bincode :: Encode ::
        encode(&self.match_words, encoder) ?; core :: result :: Result ::
        Ok(())
    }
}