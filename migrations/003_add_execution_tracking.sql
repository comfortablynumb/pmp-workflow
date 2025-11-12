-- Add updated_at column to workflow_executions
ALTER TABLE workflow_executions
ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW();

-- Add updated_at column to node_executions
ALTER TABLE node_executions
ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW();

-- Create trigger to automatically update updated_at on workflow_executions
CREATE OR REPLACE FUNCTION update_workflow_execution_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER workflow_executions_updated_at
    BEFORE UPDATE ON workflow_executions
    FOR EACH ROW
    EXECUTE FUNCTION update_workflow_execution_updated_at();

-- Create trigger to automatically update updated_at on node_executions
CREATE OR REPLACE FUNCTION update_node_execution_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER node_executions_updated_at
    BEFORE UPDATE ON node_executions
    FOR EACH ROW
    EXECUTE FUNCTION update_node_execution_updated_at();
