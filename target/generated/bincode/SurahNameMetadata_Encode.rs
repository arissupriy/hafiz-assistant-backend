impl :: bincode :: Encode for SurahNameMetadata
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.id, encoder) ?; :: bincode ::
        Encode :: encode(&self.name_simple, encoder) ?; :: bincode :: Encode
        :: encode(&self.name_arabic, encoder) ?; :: bincode :: Encode ::
        encode(&self.revelation_order, encoder) ?; :: bincode :: Encode ::
        encode(&self.revelation_place, encoder) ?; :: bincode :: Encode ::
        encode(&self.verses_count, encoder) ?; :: bincode :: Encode ::
        encode(&self.bismillah_pre, encoder) ?; core :: result :: Result ::
        Ok(())
    }
}