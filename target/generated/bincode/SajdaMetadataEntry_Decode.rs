impl < __Context > :: bincode :: Decode < __Context > for SajdaMetadataEntry
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            sajdah_number : :: bincode :: Decode :: decode(decoder) ?,
            verse_key : :: bincode :: Decode :: decode(decoder) ?, sajdah_type
            : :: bincode :: Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for SajdaMetadataEntry
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            sajdah_number : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, verse_key : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, sajdah_type : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}