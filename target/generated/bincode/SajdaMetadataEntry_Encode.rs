impl :: bincode :: Encode for SajdaMetadataEntry
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.sajdah_number, encoder) ?; ::
        bincode :: Encode :: encode(&self.verse_key, encoder) ?; :: bincode ::
        Encode :: encode(&self.sajdah_type, encoder) ?; core :: result ::
        Result :: Ok(())
    }
}