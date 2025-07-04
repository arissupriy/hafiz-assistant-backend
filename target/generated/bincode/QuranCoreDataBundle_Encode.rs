impl :: bincode :: Encode for QuranCoreDataBundle
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.pages_content, encoder) ?; ::
        bincode :: Encode :: encode(&self.surah_metadata, encoder) ?; ::
        bincode :: Encode :: encode(&self.juz_metadata, encoder) ?; :: bincode
        :: Encode :: encode(&self.hizb_metadata, encoder) ?; :: bincode ::
        Encode :: encode(&self.manzil_metadata, encoder) ?; :: bincode ::
        Encode :: encode(&self.rub_metadata, encoder) ?; :: bincode :: Encode
        :: encode(&self.ruku_metadata, encoder) ?; :: bincode :: Encode ::
        encode(&self.sajda_metadata, encoder) ?; :: bincode :: Encode ::
        encode(&self.phrases_data, encoder) ?; :: bincode :: Encode ::
        encode(&self.phrase_verses_map, encoder) ?; :: bincode :: Encode ::
        encode(&self.matching_ayahs_map, encoder) ?; :: bincode :: Encode ::
        encode(&self.quran_id_simple_translations, encoder) ?; :: bincode ::
        Encode :: encode(&self.indonesian_mokhtasar_translations, encoder) ?;
        :: bincode :: Encode :: encode(&self.surah_info_id, encoder) ?; ::
        bincode :: Encode :: encode(&self.topics_data, encoder) ?; :: bincode
        :: Encode :: encode(&self.word_roots, encoder) ?; :: bincode :: Encode
        :: encode(&self.word_stems, encoder) ?; :: bincode :: Encode ::
        encode(&self.transliterations, encoder) ?; core :: result :: Result ::
        Ok(())
    }
}