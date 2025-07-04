impl :: bincode :: Encode for SurahInfoIdEntry
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.surah_number, encoder) ?; ::
        bincode :: Encode :: encode(&self.surah_name, encoder) ?; :: bincode
        :: Encode :: encode(&self.text, encoder) ?; :: bincode :: Encode ::
        encode(&self.short_text, encoder) ?; core :: result :: Result ::
        Ok(())
    }
}