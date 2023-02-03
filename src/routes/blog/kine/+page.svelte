<script lang="ts">
    import { python, bash } from "svelte-highlight/languages";

    import Blog from "$lib/components/Blog.svelte";
    import Codeblock from "$lib/components/Codeblock.svelte";

    import Kine from "$lib/assets/kine.png";
	import ChromeImage1 from '$lib/assets/chrome_P5Xo6HKINX.png';
    import ChromeImage2 from "$lib/assets/chrome_qEPtgq1Ep4.png";
    import ChromeImage3 from "$lib/assets/chrome_68jVM6k04Z.gif";

</script>

<Blog title="Why I Made Kine">
    <img class="full-width-image" src={ Kine } width="50%" alt="Logo of Kine"/>
    <p>
        <a class="link" href="https://github.com/zomatree/kine">Link to the Github.</a>
    </p>
    <p>
        Python has been used for making web apps for a long time, Instagram for example is a giant Django program and people continue to make websites using frameworks such as Flask and Django,
        however most of these options rely on using static HTML templates, which are cumbersome to work with, they only allow passing in static values and hard to update, you must either refresh
        the page which is jarring for the user or require using javascript which makes using python for the website not worth it, at this point it would be easier to start making your web app in
        a javascript framework instead.
    </p>
    <p>
        This is why i set out to create Kine.
    </p>
    <h2>
        What is Kine?
    </h2>
    <p>
        The name Kine is short for kinetic, as chemical reactions can be achieved with sufficient kinetic energy, and reactivity is a large part of Kine. Kine is a framework i created to bring what
        creating interactive web apps should be like,
        no more HTML templates, instead it takes what libraries such as ReactJS do with components and diffing nodes. This means the entire website can be updated without needing to refresh or use
        javascript to fetch new infomation and manually update the content of elements.
    </p>
    <h2>
        A simple example program
    </h2>
    <p>
        Making a program in Kine is very easy, it can be done in under 20 lines!

        <Codeblock language={ python } code={`from kine import *
from kine.renderers.web import *

import asyncio

@component
def app(cx: Scope):
    return cx.render(p["Hello World!"])

asyncio.run(start_web(app()))`}/>

        This code doesnt do much, all it does is display "Hello World!" in the browser but its a simple example and shows off the basic structure of a program.
    </p>

    <p>
        The example is using the <span class="inline-codeblock">web</span> backend which means the python code is ran on the server and uses a websocket to send the updates to the client, this is useful if you
        need to interact with a database or other sensative data.
    </p>

    <h2>What makes Kine standout</h2>

    <p>
        Recently there have been a couple other libraries which do similar things to Kine, <a class="link" href="https://github.com/pynecone-io/pynecone">Pynecone</a> and
        <a class="link" href="https://github.com/zauberzeug/nicegui/">NiceGUI</a>
        have both seen large traction, however these libraries tend to shy away from giving you control and vary in performance.
    </p>
    <p>
        A feature which makes Kine standout is the ability to compile the entire project into a WebAssembly app, meaning that you can convert an entire python web page into a static client side page, this is great
        for when you dont need to connect directly to a database or dont want to be bogged down by being required to run a server.
    </p>

    <h2>How to make a Kine WebAssembly app</h2>
    <p>
        The steps to make a WebAssembly app is very simple, Kine comes with a CLI which can be used to create the project and build the project into the static content

        <Codeblock language={ bash } code={`\
python -m pip install kine      # installs the library
python -m kine new my_kine_app  # creates the project
cd my_kine_app                  # navigates to the project folder
python -m kine build            # build the project
python -m kine serve            # runs a web server to serve the built web app`}/>

        After this you can head to your browser similar to before and the entire python app is ran on the browser.

    </p>
    <img class="full-width-image" src = { ChromeImage1 }/>
    <p>
        From here you can use the variety of elements, components and hooks to build up the website.
    </p>
    <p>
        For example lets make a simple button and show the amount of times i pressed. Lets start with making the nodes then make it update with state.
    </p>
    <Codeblock language={ python } code={`\
@component
def app(cx: Scope):
    return cx.render(div[
        button[
            "Click Me!"
        ],
        p[
            "0"
        ]
    ])`}/>
    <img class="full-width-image" src = { ChromeImage2 }/>
    <p>
        Both are shown but the button doesnt update the number, lets fix that.
    </p>
    <Codeblock language={ python } code={`\
@component
def app(cx: Scope):
    value = use_state(cx, lambda: 0)  # create a local state with a default value of 0

    return cx.render(div[
        button(
            onclick=lambda _: value.modify(lambda v: v + 1)  # create an onclick event which modifies the state to add 1
        )[
            "Click Me!"
        ],
        p[
            f"{value.get()}"  # show the current value of the state
        ]
    ])`}/>
    <p>
        Now when we run this again and press the button it will automatically incremement the counter.
    </p>
    <img class="full-width-image" src = { ChromeImage3 }/>
    <p>
        Because Kine runs the function everytime an update is required, we need to be careful to not cause side-effects, this is why <span class="inline-codeblock">use_state</span>
        takes a function instead of the inital value as-is - not all state is as simple as an integer.
    </p>
    <p>
        <span class="inline-codeblock">use_state</span> uses <span class="inline-codeblock">use_hook</span> internally which is the building block for making all hooks in Kine, it runs the function given to it and cache the value for future reruns of the component
    </p>
    <Codeblock language={ python } code={`\
@component
def app(cx: Scope):
    x = cx.use_hook(lambda: 2 + 2)  # intensive math - we only want to calculate it once

    return cx.render(p[f"{x}"])`}/>
    <p>Other hooks exist as well, <span class="inline-codeblock">use_future</span> can be used to run coroutines</p>
</Blog>
