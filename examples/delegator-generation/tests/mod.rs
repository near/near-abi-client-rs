use delegator_generation::run;

#[tokio::test]
async fn test_client() -> anyhow::Result<()> {
    assert_eq!(run(1, 2, 3, 4).await?, (4, 6));

    Ok(())
}
