#ifndef LIB_RUBY_PARSER_TOKEN_CUSTOM_DECODER_H
#define LIB_RUBY_PARSER_TOKEN_CUSTOM_DECODER_H

#include <string>
#include <vector>
#include <variant>
#include "bytes.h"

namespace lib_ruby_parser
{
    class CustomDecoder
    {
    public:
        class Result
        {
        public:
            std::variant<Bytes, std::string> data;

            Result() = delete;
            explicit Result(Bytes output);
            explicit Result(std::string error_message);
            Result(Result &&) = default;
            Result(const Result &) = delete;

            static Result Ok(Bytes output);
            static Result Error(std::string error_message);
        };

        virtual Result rewrite(std::string encoding, Bytes input) = 0;
        virtual ~CustomDecoder(){};
    };

} // namespace lib_ruby_parser

#endif // LIB_RUBY_PARSER_TOKEN_CUSTOM_DECODER_H
