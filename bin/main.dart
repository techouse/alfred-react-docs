import 'dart:io' show exitCode, stdout;

import 'package:alfred_workflow/alfred_workflow.dart'
    show
        AlfredItem,
        AlfredItemIcon,
        AlfredItemText,
        AlfredItems,
        AlfredWorkflow;
import 'package:algolia/algolia.dart' show AlgoliaQuerySnapshot;
import 'package:args/args.dart' show ArgParser, ArgResults;
import 'package:html_unescape/html_unescape.dart';

import 'src/extensions/string_helpers.dart' show StringHelpers;
import 'src/models/search_result.dart' show SearchResult;
import 'src/services/algolia_search.dart' show AlgoliaSearch;

final HtmlUnescape unescape = HtmlUnescape();

final AlfredWorkflow workflow = AlfredWorkflow();

bool verbose = false;

void main(List<String> arguments) async {
  try {
    exitCode = 0;

    workflow.clearItems();

    final ArgParser parser = ArgParser()
      ..addOption('query', abbr: 'q', mandatory: true)
      ..addFlag('verbose', abbr: 'v', defaultsTo: false);
    final ArgResults args = parser.parse(arguments);

    verbose = args['verbose'];

    final String queryString =
        args['query'].replaceAll(RegExp(r'\s+'), ' ').trim();

    if (verbose) stdout.writeln('Query: "$queryString"');

    if (queryString.isEmpty) {
      _showPlaceholder();
    } else {
      workflow.cacheKey = queryString;
      if (await workflow.getItems() == null) {
        await _performSearch(queryString);
      }
    }
  } on FormatException catch (err) {
    exitCode = 2;
    workflow.addItem(AlfredItem(title: err.toString()));
  } catch (err) {
    exitCode = 1;
    workflow.addItem(AlfredItem(title: err.toString()));
    if (verbose) {
      rethrow;
    }
  } finally {
    workflow.run();
  }
}

void _showPlaceholder() {
  workflow.addItem(
    const AlfredItem(
      title: 'Search the React docs...',
      icon: AlfredItemIcon(path: 'icon.png'),
    ),
  );
}

Future<void> _performSearch(String query) async {
  final AlgoliaQuerySnapshot snapshot = await AlgoliaSearch.query(query);

  if (snapshot.nbHits > 0) {
    final AlfredItems items = AlfredItems(
      snapshot.hits.map((snapshot) => SearchResult.fromJson(snapshot.data)).map(
        (result) {
          final int level = int.tryParse(result.type.substring(3)) ?? 0;
          final String? title = result.hierarchy.getLevel(level);
          final Map<String, String?> hierarchy = result.hierarchy.toJson()
            ..removeWhere((_, value) => value == null || value == title);

          return AlfredItem(
            uid: result.objectID,
            title: title!,
            subtitle: level > 0
                ? unescape.convert(hierarchy.values.join(' > ')).truncate(75)
                : '',
            arg: result.url,
            text: AlfredItemText(
              largeType: title,
              copy: result.url,
            ),
            quickLookUrl: result.url,
            icon: AlfredItemIcon(path: 'icon.png'),
            valid: true,
          );
        },
      ).toList(),
    );
    workflow.addItems(items.items);
  } else {
    final Uri url =
        Uri.https('www.google.com', '/search', {'q': 'React $query'});

    workflow.addItem(
      AlfredItem(
        title: 'No matching answers found',
        subtitle: 'Shall I try and search Google?',
        arg: url.toString(),
        text: AlfredItemText(
          copy: url.toString(),
        ),
        quickLookUrl: url.toString(),
        icon: AlfredItemIcon(path: 'google.png'),
        valid: true,
      ),
    );
  }
}
