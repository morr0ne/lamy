mod aliases_in_block_sequence;
mod aliases_in_explicit_block_mapping;
mod aliases_in_flow_objects;
mod aliases_in_implicit_block_mapping;
mod allowed_characters_in_alias;
mod allowed_characters_in_keys;
mod allowed_characters_in_plain_scalars;
mod allowed_characters_in_quoted_mapping_key;
mod anchor_and_alias_as_mapping_key;
mod anchor_before_sequence_entry_on_same_line;
mod anchor_before_zero_indented_sequence;
mod anchor_for_empty_node;
mod anchor_plus_alias;
mod anchor_with_colon_in_the_middle;
mod anchor_with_unicode_character;
mod anchors_and_tags;
mod anchors_in_mapping;
mod anchors_on_empty_scalars;
mod anchors_with_colon_in_name;
mod backslashes_in_singlequotes;
mod bad_indentation_in_mapping;
mod bad_indentation_in_mapping_2;
mod bare_document_after_document_end_marker;
mod blank_lines;
mod block_mapping_with_missing_keys;
mod block_mapping_with_missing_values;
mod block_mapping_with_multiline_scalars;
mod block_mappings_in_block_sequence;
mod block_scalar_indicator_order;
mod block_scalar_keep;
mod block_scalar_strip;
mod block_scalar_strip_1_3;
mod block_scalar_with_more_spaces_than_first_content_line;
mod block_scalar_with_wrong_indented_line_after_spaces_only;
mod block_sequence_in_block_mapping;
mod block_sequence_in_block_sequence;
mod block_sequence_indentation;
mod block_submapping;
mod colon_and_adjacent_value_after_comment_on_next_line;
mod colon_and_adjacent_value_on_next_line;
mod colon_at_the_beginning_of_adjacent_flow_scalar;
mod colon_followed_by_comma;
mod colon_in_double_quoted_string;
mod comment_and_document_end_marker;
mod comment_between_plain_scalar_lines;
mod comment_in_flow_sequence_before_comma;
mod comment_in_plain_multiline_value;
mod comment_that_looks_like_a_mapping_key;
mod comment_without_whitespace_after_block_scalar_indicator;
mod comment_without_whitespace_after_doublequoted_scalar;
mod construct_binary;
mod dash_in_flow_sequence;
mod directive_by_itself_with_no_document;
mod directive_variants;
mod directive_without_document;
mod document_end_marker;
mod document_start_on_last_line;
mod document_with_footer;
mod double_quoted_scalar_with_escaped_single_quote;
mod double_quoted_string_without_closing_quote;
mod doublequoted_scalar_starting_with_a_tab;
mod duplicate_yaml_directive;
mod empty_flow_collections;
mod empty_implicit_key_in_single_pair_flow_sequences;
mod empty_keys_in_block_and_flow_mapping;
mod empty_lines_at_end_of_document;
mod empty_lines_between_mapping_elements;
mod empty_stream;
mod escaped_slash_in_double_quotes;
mod explicit_key_and_value_seperated_by_comment;
mod explicit_non_specific_tag;
mod explicit_non_specific_tag_1_3;
mod extra_words_on_yaml_directive;
mod flow_collections_over_many_lines;
mod flow_mapping;
mod flow_mapping_colon_on_line_after_key;
mod flow_mapping_edge_cases;
mod flow_mapping_in_block_sequence;
mod flow_mapping_key_on_two_lines;
mod flow_mapping_missing_a_separating_comma;
mod flow_mapping_separate_values;
mod flow_sequence;
mod flow_sequence_in_block_mapping;
mod flow_sequence_in_flow_mapping;
mod flow_sequence_in_flow_sequence;
mod flow_sequence_with_invalid_comma_at_the_beginning;
mod flow_sequence_with_invalid_extra_closing_bracket;
mod flow_sequence_with_invalid_extra_comma;
mod flow_sequence_without_closing_bracket;
mod folded_block_scalar;
mod folded_block_scalar_1_3;
mod implicit_flow_mapping_key_on_one_line;
mod implicit_key_followed_by_newline;
mod implicit_key_followed_by_newline_and_adjacent_value;
mod inline_tabs_in_double_quoted;
mod invalid_anchor_in_zero_indented_sequence;
mod invalid_block_mapping_key_on_same_line_as_previous_key;
mod invalid_comma_in_tag;
mod invalid_comment_after_comma;
mod invalid_comment_after_end_of_flow_sequence;
mod invalid_content_after_document_end_marker;
mod invalid_document_end_marker_in_single_quoted_string;
mod invalid_document_markers_in_flow_style;
mod invalid_document_start_marker_in_doublequoted_tring;
mod invalid_escape_in_double_quoted_string;
mod invalid_item_after_end_of_flow_sequence;
mod invalid_mapping_after_sequence;
mod invalid_mapping_in_plain_multiline;
mod invalid_mapping_in_plain_scalar;
mod invalid_mapping_in_plain_single_line_value;
mod invalid_nested_mapping;
mod invalid_scalar_after_sequence;
mod invalid_scalar_at_the_end_of_mapping;
mod invalid_scalar_at_the_end_of_sequence;
mod invalid_sequene_item_on_same_line_as_previous_item;
mod invalid_tabs_as_indendation_in_a_mapping;
mod invalid_tag;
mod invalid_text_after_block_scalar_indicator;
mod invalid_value_after_mapping;
mod key_with_anchor_after_missing_explicit_mapping_value;
mod leading_tab_content_in_literals;
mod leading_tabs_in_double_quoted;
mod legal_tab_after_indentation;
mod literal_block_scalar;
mod literal_block_scalar_with_more_spaces_in_first_line;
mod literal_modifers;
mod literal_scalars;
mod literal_unicode;
mod lookahead_test_cases;
mod mapping_key_and_flow_sequence_item_anchors;
mod mapping_starting_at_line;
mod mapping_with_anchor_on_document_start_line;
mod missing_colon;
mod missing_comma_in_flow;
mod missing_document_end_marker_before_directive;
mod mixed_block_mapping_explicit_to_implicit;
mod mixed_block_mapping_implicit_to_explicit;
mod more_indented_lines_at_the_beginning_of_folded_block_scalars;
mod multi_level_mapping_indent;
mod multiline_double_quoted_flow_mapping_key;
mod multiline_double_quoted_implicit_keys;
mod multiline_doublequoted_flow_mapping_key_without_value;
mod multiline_implicit_keys;
mod multiline_plain_flow_mapping_key;
mod multiline_plain_flow_mapping_key_without_value;
mod multiline_plain_scalar_with_empty_line;
mod multiline_plain_value_with_tabs_on_empty_lines;
mod multiline_scalar_at_top_level;
mod multiline_scalar_at_top_level_1_3;
mod multiline_scalar_in_mapping;
mod multiline_scalar_that_looks_like_a_yaml_directive;
mod multiline_single_quoted_implicit_keys;
mod multiline_unidented_double_quoted_block_key;
mod multiple_entry_block_sequence;
mod multiple_pair_block_mapping;
mod need_document_footer_before_directives;
mod nested_flow_collections;
mod nested_flow_collections_on_one_line;
mod nested_flow_mapping_sequence_and_mappings;
mod nested_implicit_complex_keys;
mod nested_top_level_flow_mapping;
mod node_anchor_and_tag_on_seperate_lines;
mod node_anchor_in_sequence;
mod node_anchor_not_indented;
mod node_and_mapping_key_anchors;
mod node_and_mapping_key_anchors_1_3;
mod non_specific_tags_on_scalars;
mod plain_dashes_in_flow_sequence;
mod plain_mapping_key_ending_with_colon;
mod plain_scalar_looking_like_key_comment_anchor_and_tag;
mod plain_scalar_with_backslashes;
mod plain_url_in_flow_mapping;
mod question_mark_at_start_of_flow_key;
mod question_mark_edge_cases;
mod question_marks_in_scalars;
mod scalar_doc_with_in_content;
mod scalar_value_with_two_anchors;
mod scalars_in_flow_start_with_syntax_char;
mod scalars_on_line;
mod sequence_entry_that_looks_like_two_with_wrong_indentation;
mod sequence_indent;
mod sequence_on_same_line_as_mapping_key;
mod sequence_with_same_indentation_as_parent_mapping;
mod simple_mapping_indent;
mod single_block_sequence_with_anchor;
mod single_block_sequence_with_anchor_and_explicit_document_start;
mod single_character_streams;
mod single_entry_block_sequence;
mod single_pair_block_mapping;
mod single_pair_implicit_entries;
mod spec_example_2_10_node_for_sammy_sosa_appears_twice_in_this_document;
mod spec_example_2_11_mapping_between_sequences;
mod spec_example_2_12_compact_nested_mapping;
mod spec_example_2_13_in_literals_newlines_are_preserved;
mod spec_example_2_14_in_the_folded_scalars_newlines_become_spaces;
mod spec_example_2_15_folded_newlines_are_preserved_for_more_indented_and_blank_lines;
mod spec_example_2_16_indentation_determines_scope;
mod spec_example_2_17_quoted_scalars;
mod spec_example_2_18_multi_line_flow_scalars;
mod spec_example_2_1_sequence_of_scalars;
mod spec_example_2_24_global_tags;
mod spec_example_2_25_unordered_sets;
mod spec_example_2_26_ordered_mappings;
mod spec_example_2_27_invoice;
mod spec_example_2_28_log_file;
mod spec_example_2_2_mapping_scalars_to_scalars;
mod spec_example_2_3_mapping_scalars_to_sequences;
mod spec_example_2_4_sequence_of_mappings;
mod spec_example_2_5_sequence_of_sequences;
mod spec_example_2_6_mapping_of_mappings;
mod spec_example_2_7_two_documents_in_a_stream;
mod spec_example_2_8_play_by_play_feed_from_a_game;
mod spec_example_2_9_single_document_with_two_comments;
mod spec_example_5_12_tabs_and_spaces;
mod spec_example_5_3_block_structure_indicators;
mod spec_example_5_4_flow_collection_indicators;
mod spec_example_5_5_comment_indicator;
mod spec_example_5_6_node_property_indicators;
mod spec_example_5_7_block_scalar_indicators;
mod spec_example_5_8_quoted_scalar_indicators;
mod spec_example_5_9_directive_indicator;
mod spec_example_6_10_comment_lines;
mod spec_example_6_11_multi_line_comments;
mod spec_example_6_12_separation_spaces;
mod spec_example_6_13_reserved_directives;
mod spec_example_6_13_reserved_directives_1_3;
mod spec_example_6_14_yaml_directive;
mod spec_example_6_16_tag_directive;
mod spec_example_6_18_primary_tag_handle;
mod spec_example_6_18_primary_tag_handle_1_3;
mod spec_example_6_19_secondary_tag_handle;
mod spec_example_6_1_indentation_spaces;
mod spec_example_6_20_tag_handles;
mod spec_example_6_21_local_tag_prefix;
mod spec_example_6_22_global_tag_prefix;
mod spec_example_6_23_node_properties;
mod spec_example_6_24_verbatim_tags;
mod spec_example_6_26_tag_shorthands;
mod spec_example_6_28_non_specific_tags;
mod spec_example_6_29_node_anchors;
mod spec_example_6_2_indentation_indicators;
mod spec_example_6_3_separation_spaces;
mod spec_example_6_4_line_prefixes;
mod spec_example_6_5_empty_lines;
mod spec_example_6_5_empty_lines_1_3;
mod spec_example_6_6_line_folding;
mod spec_example_6_6_line_folding_1_3;
mod spec_example_6_7_block_folding;
mod spec_example_6_8_flow_folding;
mod spec_example_6_8_flow_folding_1_3;
mod spec_example_6_9_separated_comment;
mod spec_example_7_10_plain_characters;
mod spec_example_7_11_plain_implicit_keys;
mod spec_example_7_12_plain_lines;
mod spec_example_7_13_flow_sequence;
mod spec_example_7_14_flow_sequence_entries;
mod spec_example_7_15_flow_mappings;
mod spec_example_7_16_flow_mapping_entries;
mod spec_example_7_18_flow_mapping_adjacent_values;
mod spec_example_7_19_single_pair_flow_mappings;
mod spec_example_7_1_alias_nodes;
mod spec_example_7_20_single_pair_explicit_entry;
mod spec_example_7_23_flow_content;
mod spec_example_7_24_flow_nodes;
mod spec_example_7_2_empty_content;
mod spec_example_7_3_completely_empty_flow_nodes;
mod spec_example_7_4_double_quoted_implicit_keys;
mod spec_example_7_5_double_quoted_line_breaks;
mod spec_example_7_5_double_quoted_line_breaks_1_3;
mod spec_example_7_6_double_quoted_lines;
mod spec_example_7_6_double_quoted_lines_1_3;
mod spec_example_7_7_single_quoted_characters;
mod spec_example_7_7_single_quoted_characters_1_3;
mod spec_example_7_8_single_quoted_implicit_keys;
mod spec_example_7_9_single_quoted_lines;
mod spec_example_7_9_single_quoted_lines_1_3;
mod spec_example_8_10_folded_lines_8_13_final_empty_lines;
mod spec_example_8_14_block_sequence;
mod spec_example_8_15_block_sequence_entry_types;
mod spec_example_8_16_block_mappings;
mod spec_example_8_17_explicit_block_mapping_entries;
mod spec_example_8_18_implicit_block_mapping_entries;
mod spec_example_8_19_compact_block_mappings;
mod spec_example_8_1_block_scalar_header;
mod spec_example_8_20_block_node_types;
mod spec_example_8_21_block_scalar_nodes;
mod spec_example_8_21_block_scalar_nodes_1_3;
mod spec_example_8_22_block_collection_nodes;
mod spec_example_8_2_block_indentation_indicator;
mod spec_example_8_2_block_indentation_indicator_1_3;
mod spec_example_8_4_chomping_final_line_break;
mod spec_example_8_5_chomping_trailing_lines;
mod spec_example_8_6_empty_scalar_chomping;
mod spec_example_8_7_literal_scalar;
mod spec_example_8_7_literal_scalar_1_3;
mod spec_example_8_8_literal_content;
mod spec_example_8_8_literal_content_1_3;
mod spec_example_8_9_folded_scalar;
mod spec_example_8_9_folded_scalar_1_3;
mod spec_example_9_2_document_markers;
mod spec_example_9_3_bare_documents;
mod spec_example_9_4_explicit_documents;
mod spec_example_9_5_directives_documents;
mod spec_example_9_6_stream;
mod spec_example_9_6_stream_1_3;
mod syntax_character_edge_cases;
mod tab_after_document_header;
mod tab_at_beginning_of_line_followed_by_a_flow_mapping;
mod tab_indented_top_flow;
mod tabs_in_various_contexts;
mod tabs_that_look_like_indentation;
mod tag_shorthand_used_in_documents_but_only_defined_in_the_first;
mod tags_for_block_objects;
mod tags_for_flow_objects;
mod tags_for_root_objects;
mod tags_in_block_sequence;
mod tags_in_explicit_mapping;
mod tags_in_implicit_mapping;
mod tags_on_empty_scalars;
mod three_dashes_and_content_without_space;
mod three_dashes_and_content_without_space_1_3;
mod three_explicit_integers_in_a_block_sequence;
mod trailing_comment_in_multiline_plain_scalar;
mod trailing_content_after_quoted_value;
mod trailing_content_that_looks_like_a_mapping;
mod trailing_line_of_spaces;
mod trailing_spaces_after_flow_collection;
mod trailing_tabs_in_double_quoted;
mod trailing_whitespace_in_streams;
mod two_document_start_markers;
mod two_scalar_docs_with_trailing_comments;
mod various_combinations_of_explicit_block_mappings;
mod various_combinations_of_tags_and_anchors;
mod various_empty_or_newline_only_quoted_strings;
mod various_location_of_anchors_in_flow_sequence;
mod various_trailing_comments;
mod various_trailing_comments_1_3;
mod various_trailing_tabs;
mod whitespace_after_scalars_in_flow;
mod whitespace_around_colon_in_mappings;
mod wrong_indendation_in_map;
mod wrong_indendation_in_mapping;
mod wrong_indendation_in_sequence;
mod wrong_indented_flow_sequence;
mod wrong_indented_multiline_quoted_scalar;
mod wrong_indented_sequence_item;
mod yaml_directive_without_document_end_marker;
mod zero_indented_block_scalar;
mod zero_indented_block_scalar_with_line_that_looks_like_a_comment;
mod zero_indented_sequences_in_explicit_mapping_keys;
