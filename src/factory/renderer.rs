use crate::design::BlockchainDesign;
use std::path::PathBuf;

pub struct TemplateRenderer;

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Template not found: {0}")]
    TemplateNotFound(String),
}

impl TemplateRenderer {
    pub fn new() -> Result<Self, RenderError> {
        Ok(Self)
    }

    pub fn render_all(
        &self,
        design: &BlockchainDesign,
    ) -> Result<Vec<(PathBuf, String)>, RenderError> {
        let mut files = Vec::new();

        let templates = [
            ("src/lib.rs", include_str!("../templates/lib.rs.hbs")),
            ("src/main.rs", include_str!("../templates/main.rs.hbs")),
            ("src/types.rs", include_str!("../templates/types.rs.hbs")),
            ("src/crypto.rs", include_str!("../templates/crypto.rs.hbs")),
            (
                "src/storage.rs",
                include_str!("../templates/storage.rs.hbs"),
            ),
            (
                "src/consensus.rs",
                include_str!("../templates/consensus.rs.hbs"),
            ),
            ("src/chain.rs", include_str!("../templates/chain.rs.hbs")),
            ("src/miner.rs", include_str!("../templates/miner.rs.hbs")),
            ("src/p2p.rs", include_str!("../templates/p2p.rs.hbs")),
            ("src/cli.rs", include_str!("../templates/cli.rs.hbs")),
        ];

        for (output_path, template_content) in &templates {
            let rendered = self.render_template(template_content, design);
            files.push((PathBuf::from(output_path), rendered));
        }

        Ok(files)
    }

    fn render_template(&self, template: &str, design: &BlockchainDesign) -> String {
        let c = &design.consensus;
        let n = &design.network;

        template
            .replace("{{design_name}}", &design.design_name)
            .replace("{{description}}", &design.description)
            .replace("{{consensus.scrypt_n}}", &c.scrypt_n.to_string())
            .replace("{{consensus.scrypt_r}}", &c.scrypt_r.to_string())
            .replace("{{consensus.scrypt_p}}", &c.scrypt_p.to_string())
            .replace("{{consensus.scrypt_len}}", &c.scrypt_len.to_string())
            .replace(
                "{{consensus.target_block_time_secs}}",
                &c.target_block_time_secs.to_string(),
            )
            .replace(
                "{{consensus.max_payloads_per_block}}",
                &c.max_payloads_per_block.to_string(),
            )
            .replace(
                "{{consensus.payload_size_bytes}}",
                &c.payload_size_bytes.to_string(),
            )
            .replace(
                "{{consensus.blocks_per_year}}",
                &c.blocks_per_year.to_string(),
            )
            .replace(
                "{{consensus.difficulty_adjustment_period_blocks}}",
                &c.difficulty_adjustment_period_blocks.to_string(),
            )
            .replace(
                "{{consensus.difficulty_vote_window_blocks}}",
                &c.difficulty_vote_window_blocks.to_string(),
            )
            .replace("{{consensus.block_reward}}", &c.block_reward.to_string())
            .replace(
                "{{consensus.reward_decimals}}",
                &c.reward_decimals.to_string(),
            )
            .replace("{{consensus.max_supply}}", &c.max_supply.to_string())
            .replace(
                "{{consensus.prune_depth_blocks}}",
                &c.prune_depth_blocks.to_string(),
            )
            .replace("{{network.p2p_protocol_version}}", &n.p2p_protocol_version)
            .replace("{{network.listen_port}}", &n.listen_port.to_string())
            .replace(
                "{{network.gossipsub_heartbeat_secs}}",
                &n.gossipsub_heartbeat_secs.to_string(),
            )
    }
}

impl Default for TemplateRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create TemplateRenderer")
    }
}
