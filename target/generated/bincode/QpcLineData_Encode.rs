impl :: bincode :: Encode for QpcLineData
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.page_number, encoder) ?; ::
        bincode :: Encode :: encode(&self.line_number, encoder) ?; :: bincode
        :: Encode :: encode(&self.line_type, encoder) ?; :: bincode :: Encode
        :: encode(&self.is_centered, encoder) ?; :: bincode :: Encode ::
        encode(&self.first_word_id, encoder) ?; :: bincode :: Encode ::
        encode(&self.last_word_id, encoder) ?; :: bincode :: Encode ::
        encode(&self.surah_number, encoder) ?; core :: result :: Result ::
        Ok(())
    }
}