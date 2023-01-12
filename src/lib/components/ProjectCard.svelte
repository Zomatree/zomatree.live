<script lang="ts">
    export let title: string;
    export let description: string;
    export let tag: string;
    export let tag_colour: string;
    export let url: string | null = null;
    export let github_url: string;
</script>

<style lang="scss">
    .project-card {
        background-color: var(--bg-2);
        border-radius: 10px;
        max-width: 400px;
        height: 200px;
        padding: 20px;
        display: flex;
        flex-direction: column;
    }

    .title-row {
        display: flex;
        color: var(--primary-4);
        align-items: center;
        margin-bottom: 10px;
    }

    .project-title {
        flex-grow: 1;
        margin-top: 0;
        margin-bottom: 0;
    }

    .website-link {
        color: lightblue;
        text-decoration: underline;
    }

    .project-type {
        height: min-content;
        padding: 8px 10px;
        font-size: 14px;
        font-weight: 300;
        border-radius: 50px;
        color: black;
    }

    .project-description {
        color: var(--primary-3)
    }

    .grow {
        flex-grow: 1;
    }
</style>

<div class="project-card">
    <div class="title-row">
        <h2 class="project-title">
            { title }
        </h2>

        <div class="project-type" style="background-color: {tag_colour}">
            {tag}
        </div>
    </div>

    <span class="project-description grow">
        {description}
    </span>

    <span class="project-description">
        <a class="website-link" href={github_url} target="_blank" rel="nooopener noreferrer">Github </a>
        {#if url}
        • <a class="website-link" href={url} target="_blank" rel="nooopener noreferrer">Website</a>
        {/if}
    </span>
</div>
