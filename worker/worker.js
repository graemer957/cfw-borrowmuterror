addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
    const { fromRS } = wasm_bindgen;
    await wasm_bindgen(wasm)

    if (request.url.endsWith("/js")) {
        return new Response(fromJS(), { status: 200 })
    } else if (request.url.endsWith("/rs")) {
        return new Response(fromRS(), { status: 200 })
    }

    return new Response(null, { status: 404 })
}

function fromJS() {
    return "JavaScript"
}
