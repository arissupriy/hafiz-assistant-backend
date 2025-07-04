impl < __Context > :: bincode :: Decode < __Context > for QuranCoreDataBundle
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            pages_content : :: bincode :: Decode :: decode(decoder) ?,
            surah_metadata : :: bincode :: Decode :: decode(decoder) ?,
            juz_metadata : :: bincode :: Decode :: decode(decoder) ?,
            hizb_metadata : :: bincode :: Decode :: decode(decoder) ?,
            manzil_metadata : :: bincode :: Decode :: decode(decoder) ?,
            rub_metadata : :: bincode :: Decode :: decode(decoder) ?,
            ruku_metadata : :: bincode :: Decode :: decode(decoder) ?,
            sajda_metadata : :: bincode :: Decode :: decode(decoder) ?,
            phrases_data : :: bincode :: Decode :: decode(decoder) ?,
            phrase_verses_map : :: bincode :: Decode :: decode(decoder) ?,
            matching_ayahs_map : :: bincode :: Decode :: decode(decoder) ?,
            quran_id_simple_translations : :: bincode :: Decode ::
            decode(decoder) ?, indonesian_mokhtasar_translations : :: bincode
            :: Decode :: decode(decoder) ?, surah_info_id : :: bincode ::
            Decode :: decode(decoder) ?, topics_data : :: bincode :: Decode ::
            decode(decoder) ?, word_roots : :: bincode :: Decode ::
            decode(decoder) ?, word_stems : :: bincode :: Decode ::
            decode(decoder) ?, transliterations : :: bincode :: Decode ::
            decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for QuranCoreDataBundle
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            pages_content : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, surah_metadata : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            juz_metadata : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, hizb_metadata : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            manzil_metadata : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, rub_metadata : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            ruku_metadata : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, sajda_metadata : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            phrases_data : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, phrase_verses_map : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            matching_ayahs_map : :: bincode :: BorrowDecode ::< '_, __Context
            >:: borrow_decode(decoder) ?, quran_id_simple_translations : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, indonesian_mokhtasar_translations : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, surah_info_id : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            topics_data : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, word_roots : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, word_stems : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, transliterations : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
        })
    }
}