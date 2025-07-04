impl < __Context > :: bincode :: Decode < __Context > for QpcLineData
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            page_number : :: bincode :: Decode :: decode(decoder) ?,
            line_number : :: bincode :: Decode :: decode(decoder) ?, line_type
            : :: bincode :: Decode :: decode(decoder) ?, is_centered : ::
            bincode :: Decode :: decode(decoder) ?, first_word_id : :: bincode
            :: Decode :: decode(decoder) ?, last_word_id : :: bincode ::
            Decode :: decode(decoder) ?, surah_number : :: bincode :: Decode
            :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for QpcLineData
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            page_number : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, line_number : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, line_type : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, is_centered : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, first_word_id : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, last_word_id : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            surah_number : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}