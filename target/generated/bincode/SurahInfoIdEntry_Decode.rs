impl < __Context > :: bincode :: Decode < __Context > for SurahInfoIdEntry
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            surah_number : :: bincode :: Decode :: decode(decoder) ?,
            surah_name : :: bincode :: Decode :: decode(decoder) ?, text : ::
            bincode :: Decode :: decode(decoder) ?, short_text : :: bincode ::
            Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for SurahInfoIdEntry
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            surah_number : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, surah_name : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, text : :: bincode
            :: BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            short_text : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}