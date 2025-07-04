impl < __Context > :: bincode :: Decode < __Context > for AyahMetadata
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            id : :: bincode :: Decode :: decode(decoder) ?, surah_number : ::
            bincode :: Decode :: decode(decoder) ?, ayah_number : :: bincode
            :: Decode :: decode(decoder) ?, verse_key : :: bincode :: Decode
            :: decode(decoder) ?, text : :: bincode :: Decode ::
            decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for AyahMetadata
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            id : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, surah_number : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            ayah_number : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, verse_key : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, text : :: bincode
            :: BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
        })
    }
}