
makeEdit cursor desc =
    case desc of
        OneOf details ->
            case cursor.makeEdit cursor.indentation (foundStart details.child) desc of
                NoIdFound ->
                    -- dive further
                    case makeFoundEdit (increaseIndent cursor) details.child of
                        EditMade maybeSeed maybePush newFound ->
                            EditMade maybeSeed
                                maybePush
                                (OneOf
                                    { details
                                        | child = expandFound maybePush newFound
                                    }
                                )

                        NoIdFound ->
                            NoIdFound

                        ErrorMakingEdit err ->
                            ErrorMakingEdit err

                otherwise ->
                    otherwise