impl :: bincode :: Encode for JuzMetadata
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.juz_number, encoder) ?; ::
        bincode :: Encode :: encode(&self.verses_count, encoder) ?; :: bincode
        :: Encode :: encode(&self.first_verse_key, encoder) ?; :: bincode ::
        Encode :: encode(&self.last_verse_key, encoder) ?; :: bincode ::
        Encode :: encode(&self.verse_mapping, encoder) ?; core :: result ::
        Result :: Ok(())
    }
}