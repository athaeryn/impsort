impsort 😈
=======

_impsort sorts your ES6 imports_

---

`impsort` sorts imports by stripping out noise like `import` and curly braces.
It also treats `import * as actions` as `import actions` so that sorting
operates on the actual identifier being imported.

## Example

```js
// Before
import { takeEvery, takeLatest } from 'redux-saga'
import actions from './actions'
import { fork } from 'redux-saga/effects'
import * as selectors from 'selectors'

// After
import actions from './actions'
import { fork } from 'redux-saga/effects'
import * as selectors from 'selectors'
import { takeEvery, takeLatest } from 'redux-saga'
```

## Why?

I don't know, maybe this is a terrible idea.
