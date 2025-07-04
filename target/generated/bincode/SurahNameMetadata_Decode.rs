impl < __Context > :: bincode :: Decode < __Context > for SurahNameMetadata
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            id : :: bincode :: Decode :: decode(decoder) ?, name_simple : ::
            bincode :: Decode :: decode(decoder) ?, name_arabic : :: bincode
            :: Decode :: decode(decoder) ?, revelation_order : :: bincode ::
            Decode :: decode(decoder) ?, revelation_place : :: bincode ::
            Decode :: decode(decoder) ?, verses_count : :: bincode :: Decode
            :: decode(decoder) ?, bismillah_pre : :: bincode :: Decode ::
            decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for SurahNameMetadata
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            id : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, name_simple : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, name_arabic : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, revelation_order : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            revelation_place : :: bincode :: BorrowDecode ::< '_, __Context
            >:: borrow_decode(decoder) ?, verses_count : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            bismillah_pre : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}