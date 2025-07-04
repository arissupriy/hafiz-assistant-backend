impl < __Context > :: bincode :: Decode < __Context > for ManzilMetadataEntry
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            manzil_number : :: bincode :: Decode :: decode(decoder) ?,
            verses_count : :: bincode :: Decode :: decode(decoder) ?,
            first_verse_key : :: bincode :: Decode :: decode(decoder) ?,
            last_verse_key : :: bincode :: Decode :: decode(decoder) ?,
            verse_mapping : :: bincode :: Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for ManzilMetadataEntry
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            manzil_number : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, verses_count : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            first_verse_key : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, last_verse_key : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            verse_mapping : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}