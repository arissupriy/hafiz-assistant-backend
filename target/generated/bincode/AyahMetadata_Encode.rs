impl :: bincode :: Encode for AyahMetadata
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.id, encoder) ?; :: bincode ::
        Encode :: encode(&self.surah_number, encoder) ?; :: bincode :: Encode
        :: encode(&self.ayah_number, encoder) ?; :: bincode :: Encode ::
        encode(&self.verse_key, encoder) ?; :: bincode :: Encode ::
        encode(&self.text, encoder) ?; core :: result :: Result :: Ok(())
    }
}