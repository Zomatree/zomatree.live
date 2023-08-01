<script lang="ts">
    export let title: string;
</script>

<style lang="scss">
    .blog-header {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    .blog-content {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    :global(.blog-content > p) {
        color: var(--primary-4);
    }

    :global(.blog-content > h2) {
        color: var(--primary-3);
    }

    .blog-post {
        padding-left: 20%;
        padding-right: 20%;
    }

    @media screen and (max-width: 823px) {
        .blog-post {
            padding-left: 10px;
            padding-right: 10px;
        }
    }
</style>

<div class="blog-post">
    <div class="blog-header">
        <h1 class="header">
            {title}
        </h1>
        <h1>
            <a href="/" class="go-home-link">Home</a>
        </h1>
    </div>

    <div class="blog-content">
        <slot/>
    </div>
</div>
