impl :: bincode :: Encode for TopicEntry
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.topic_id, encoder) ?; :: bincode
        :: Encode :: encode(&self.name, encoder) ?; :: bincode :: Encode ::
        encode(&self.arabic_name, encoder) ?; :: bincode :: Encode ::
        encode(&self.parent_id, encoder) ?; :: bincode :: Encode ::
        encode(&self.description, encoder) ?; :: bincode :: Encode ::
        encode(&self.ayahs, encoder) ?; :: bincode :: Encode ::
        encode(&self.keywords, encoder) ?; :: bincode :: Encode ::
        encode(&self.total_ayahs, encoder) ?; core :: result :: Result ::
        Ok(())
    }
}