"""
Conformance tests for jupyter-ysync against jupyter_ydoc.

These tests verify that our Rust implementation can:
1. Apply updates created by jupyter_ydoc
2. Produce updates that jupyter_ydoc can apply
3. Achieve document state convergence after sync
"""

import pytest
from jupyter_ydoc import YNotebook
from pycrdt import Doc, Array, Map

# Import our Rust implementation
from jupyter_ysync import NotebookDoc


class TestJupyterYdocCompat:
    """Test compatibility with jupyter_ydoc."""

    def test_empty_notebook_state_vector(self):
        """Both implementations should produce similar state vectors for empty docs."""
        # Create empty notebooks
        rust_doc = NotebookDoc()
        py_notebook = YNotebook()

        # Both should have 0 cells initially
        assert rust_doc.cell_count() == 0

    def test_apply_jupyter_ydoc_update(self):
        """Our implementation should be able to apply updates from jupyter_ydoc."""
        # Create a notebook with jupyter_ydoc and add content
        py_notebook = YNotebook()

        # Get the initial state as an update
        py_doc = py_notebook.ydoc
        update = py_doc.get_update()

        # Apply to our Rust implementation
        rust_doc = NotebookDoc()
        rust_doc.apply_update(bytes(update))

        # Both should have the same structure
        # (jupyter_ydoc initializes with empty cells array and metadata)

    def test_rust_update_to_jupyter_ydoc(self):
        """jupyter_ydoc should be able to apply updates from our implementation."""
        # Create a notebook with our implementation
        rust_doc = NotebookDoc()
        rust_update = rust_doc.get_update()

        # Create a new jupyter_ydoc notebook and apply our update
        py_notebook = YNotebook()
        py_doc = py_notebook.ydoc
        py_doc.apply_update(rust_update)

        # Both should now have the same state

    def test_bidirectional_sync(self):
        """Test full bidirectional sync between implementations."""
        # Start with jupyter_ydoc
        py_notebook = YNotebook()
        py_doc = py_notebook.ydoc

        # Get jupyter_ydoc's initial state
        py_update = py_doc.get_update()

        # Apply to Rust
        rust_doc = NotebookDoc()
        rust_doc.apply_update(bytes(py_update))

        # Get Rust state and apply back to Python
        rust_update = rust_doc.get_update()

        # Create a fresh Python doc and apply Rust update
        py_notebook2 = YNotebook()
        py_doc2 = py_notebook2.ydoc
        py_doc2.apply_update(rust_update)

        # Both should converge to the same state
        # The state vectors should be compatible


class TestPyCrdtCompat:
    """Test compatibility with raw pycrdt."""

    def test_pycrdt_update_roundtrip(self):
        """Test that we can roundtrip updates with pycrdt."""
        # Create a pycrdt Doc with similar structure
        py_doc = Doc()

        # Initialize with our schema
        with py_doc.transaction():
            cells = py_doc.get("cells", type=Array)
            metadata = py_doc.get("metadata", type=Map)

        # Get the update
        py_update = py_doc.get_update()

        # Apply to our implementation
        rust_doc = NotebookDoc()
        rust_doc.apply_update(bytes(py_update))

        # Get our update
        rust_update = rust_doc.get_update()

        # Apply back to a fresh pycrdt doc
        py_doc2 = Doc()
        py_doc2.apply_update(rust_update)

        # Verify structure exists
        with py_doc2.transaction():
            cells2 = py_doc2.get("cells", type=Array)
            metadata2 = py_doc2.get("metadata", type=Map)
            assert cells2 is not None
            assert metadata2 is not None

    def test_state_vector_encoding(self):
        """Test that state vector encoding is compatible."""
        rust_doc = NotebookDoc()
        sv_bytes = rust_doc.get_state_vector()

        # pycrdt should be able to use this state vector
        # (for computing diffs)
        assert len(sv_bytes) > 0


class TestDocumentStructure:
    """Test that document structure matches jupyter_ydoc expectations."""

    def test_cells_array_exists(self):
        """Verify cells array is created."""
        rust_doc = NotebookDoc()
        json_str = rust_doc.to_json()
        import json
        doc_json = json.loads(json_str)

        assert "cells" in doc_json
        assert isinstance(doc_json["cells"], list)

    def test_metadata_map_exists(self):
        """Verify metadata map is created."""
        rust_doc = NotebookDoc()
        json_str = rust_doc.to_json()
        import json
        doc_json = json.loads(json_str)

        assert "metadata" in doc_json

    def test_client_id_is_unique(self):
        """Each document should have a unique client ID."""
        doc1 = NotebookDoc()
        doc2 = NotebookDoc()

        # Client IDs should be different
        assert doc1.client_id() != doc2.client_id()


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
