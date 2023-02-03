<script lang="ts">
    import Highlight from "svelte-highlight";
    import type { Language } from "svelte-highlight/Highlight.svelte";
    import atom_one_dark from "svelte-highlight/styles/atom-one-dark";

    export let language: Language;
    export let code: string;

    export async function copyCodeToClipboard(evt: MouseEvent) {
        await navigator.clipboard.writeText(code);
    }
</script>

<svelte:head>
  {@html atom_one_dark}
</svelte:head>

<style lang="scss">
    .codeblock {
        margin-top: 10px;
        margin-bottom: 20px;
        padding: 5px;
        background-color: #282c34;
        border-radius: 5px;
        font-family: "Fira Mono";
        font-size: 14px;
        width: 100%;
    }

    .codeblock-header-item {
        background-color: #d0a34a;
        width: fit-content;
        padding: 5px;
        font-weight: 900;
        color: black;
        border-radius: 5px;
        font-family: "Fira Mono";
        margin-bottom: 2px;
    }

    .codeblock-header {
        margin: 5px 5px 0px 5px;
        display: flex;
        justify-content: space-between;
    }

    .codeblock-copy {
        border: 0;
    }

    .codeblock-copy:active {
        margin-top: 2px;
        margin-bottom: 0px;
    }

    :global(.codeblock > pre) {
        margin: 0;
    }

    :global(.hljs) {
        font-family: "Fira Mono";
        padding: 0.5em!important;
    }
</style>

<div class="codeblock">
    <div class="codeblock-header">
        <div class="codeblock-header-item">
            {language.name}
        </div>
        <button class="codeblock-copy codeblock-header-item" on:click={copyCodeToClipboard}>
            <span>Copy</span>
        </button>
    </div>
    <Highlight language={language} code={code}/>
</div>
